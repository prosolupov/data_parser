use crate::error::CustomError;
use crate::format::DataFormat;
use crate::models::{Record, Status, TxType};
use std::io;
use std::io::{Read, Write};

/// Бинарный формат
/// Чтение и создание бинарного формата
#[derive(Debug)]
pub struct BinFormat {
    pub bin_rows: Vec<Record>,
}

const YPBN: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

impl DataFormat for BinFormat {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, CustomError> {
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

            let description_raw = String::from_utf8(description_buf).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Неверный UTF-8: {}", e))
            })?;

            let description = description_raw
                .trim()
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string();

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

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), CustomError> {
        for record in &self.bin_rows {
            let quoted_description = format!("\"{}\"", record.description);
            let description_bytes = quoted_description.as_bytes();
            let desc_len = description_bytes.len() as u32;

            let record_body_size = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + desc_len as u64;

            writer.write_all(&YPBN)?;

            writer.write_all(&((record_body_size as u32).to_be_bytes()))?;

            writer.write_all(&record.tx_id.to_be_bytes())?;
            writer.write_all(&[record.tx_type.clone() as u8])?;
            writer.write_all(&record.from_user_id.to_be_bytes())?;
            writer.write_all(&record.to_user_id.to_be_bytes())?;
            writer.write_all(&record.amount.to_be_bytes())?;
            writer.write_all(&record.timestamp.to_be_bytes())?;
            writer.write_all(&[record.status.clone() as u8])?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::txt::TxtFormat;
    use std::io::Cursor;
    fn bin_record() -> Record {
        Record {
            tx_id: 1,
            from_user_id: 10,
            to_user_id: 20,
            amount: 100,
            timestamp: 123456789,
            tx_type: TxType::DEPOSIT,
            status: Status::FAILURE,
            description: "Record number 1".to_string(),
        }
    }
    #[test]
    fn bin_write_then_read() -> Result<(), CustomError> {
        let rec = bin_record();

        let mut bin = BinFormat {
            bin_rows: vec![rec.clone()],
        };

        let mut buf: Vec<u8> = Vec::new();

        {
            let mut writer = std::io::Cursor::new(&mut buf);
            bin.write_to(&mut writer).unwrap();
        }

        let mut cursor = Cursor::new(&buf);

        let parsed = BinFormat::from_read(&mut cursor)?;

        assert_eq!(parsed.bin_rows.len(), 1);
        assert_eq!(parsed.bin_rows[0], rec);

        Ok(())
    }
}
