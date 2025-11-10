use crate::format::Format;
use crate::models::{Record, Status, TxType};
use std::error::Error;
use std::io;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct BinFormat {
    pub bin_rows: Vec<Record>,
}

const YPBN: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

impl Format for BinFormat {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut bin_rows: Vec<Record> = Vec::new();

        loop {
            let mut magic_buf = [0u8; 4];
            if r.read(&mut magic_buf)? == 0 {
                break;
            }

            if magic_buf != YPBN {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Неверное магическое число заголовка",
                )
                .into());
            }

            let mut size_buf = [0u8; 4];
            r.read_exact(&mut size_buf)?;
            let record_size = u32::from_be_bytes(size_buf) as u64;

            let mut record_body_reader = r.by_ref().take(record_size);

            let mut buf_u64 = [0u8; 8];
            record_body_reader.read_exact(&mut buf_u64)?;
            let tx_id = u64::from_be_bytes(buf_u64);

            let mut buf_u8 = [0u8; 1];
            record_body_reader.read_exact(&mut buf_u8)?;
            let tx_type_raw = buf_u8[0];

            record_body_reader.read_exact(&mut buf_u64)?;
            let from_user_id = u64::from_be_bytes(buf_u64);

            record_body_reader.read_exact(&mut buf_u64)?;
            let to_user_id = u64::from_be_bytes(buf_u64);

            record_body_reader.read_exact(&mut buf_u64)?;
            let amount = u64::from_be_bytes(buf_u64);

            record_body_reader.read_exact(&mut buf_u64)?;
            let timestamp = u64::from_be_bytes(buf_u64);

            record_body_reader.read_exact(&mut buf_u8)?;
            let status_raw = buf_u8[0];

            record_body_reader.read_exact(&mut size_buf)?;
            let desc_len = u32::from_be_bytes(size_buf) as usize;

            let mut description_buf = vec![0u8; desc_len];
            record_body_reader.read_exact(&mut description_buf)?;
            let description = String::from_utf8(description_buf).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Неверный UTF-8: {}", e))
            })?;

            bin_rows.push(Record {
                tx_id,
                tx_type: TxType::from(tx_type_raw),
                from_user_id,
                to_user_id,
                amount,
                timestamp,
                status: Status::from(status_raw),
                description,
            });
        }

        Ok(Self { bin_rows })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        for record in &self.bin_rows {
            let description_bytes = record.description.as_bytes();
            let desc_len = description_bytes.len() as u32;

            let record_body_size = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + desc_len as u64;

            // --- Запись заголовка ---
            writer.write_all(&YPBN)?;
            // Преобразование размера записи в Big-Endian байты
            writer.write_all(&((record_body_size as u32).to_be_bytes()))?;

            // --- Запись тела записи ---
            writer.write_all(&record.tx_id.to_be_bytes())?;
            writer.write_all(&[record.tx_type.clone() as u8])?; // Запись одного байта
            writer.write_all(&record.from_user_id.to_be_bytes())?;
            writer.write_all(&record.to_user_id.to_be_bytes())?;
            writer.write_all(&record.amount.to_be_bytes())?; // i64 to BE bytes
            writer.write_all(&record.timestamp.to_be_bytes())?;
            writer.write_all(&[record.status.clone() as u8])?; // Запись одного байта
            writer.write_all(&desc_len.to_be_bytes())?;
            writer.write_all(description_bytes)?;
        }

        Ok(())
    }
}

impl From<Vec<Record>> for BinFormat {
    fn from(records: Vec<Record>) -> Self {
        BinFormat { bin_rows: records }
    }
}

impl From<BinFormat> for Vec<Record> {
    fn from(bin_format: BinFormat) -> Self {
        bin_format.bin_rows
    }
}
