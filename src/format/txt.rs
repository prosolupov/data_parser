use crate::error::CustomError;
use crate::format::DataFormat;
use crate::models::Record;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::{fmt, mem};

///  Текстовый формат
/// Чтение и создание текстового формата
#[derive(Debug)]
pub struct TxtFormat {
    pub txt_rows: Vec<Record>,
}

impl DataFormat for TxtFormat {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, CustomError> {
        let reader = BufReader::new(r);
        let mut txt_rows: Vec<Record> = Vec::new();

        let mut current_block_data: HashMap<String, String> = HashMap::new();

        let items: Vec<_> = reader.lines().collect();

        for item in items {
            match item {
                Ok(line) => {
                    if !line.starts_with('#') && !line.is_empty() {
                        if let Some((k, v)) = line.split_once(':') {
                            current_block_data.insert(k.trim().to_string(), v.trim().to_string());
                        }
                    } else if !current_block_data.is_empty() {
                        let record = TxtFormat::created_record(mem::take(&mut current_block_data));
                        txt_rows.push(record);
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(Self { txt_rows })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), CustomError> {
        for (i, record) in self.txt_rows.iter().enumerate() {
            writeln!(writer, "# Record {} ({:?})", i + 1, record.tx_type)?;
            writeln!(writer, "{}", record)?;
        }

        Ok(())
    }
}

impl TxtFormat {
    fn created_record(payload: HashMap<String, String>) -> Record {
        Record {
            tx_id: payload.get("TX_ID").unwrap().parse().unwrap(),
            tx_type: payload.get("TX_TYPE").unwrap().parse().unwrap(),
            from_user_id: payload.get("FROM_USER_ID").unwrap().parse().unwrap(),
            to_user_id: payload.get("TO_USER_ID").unwrap().parse().unwrap(),
            amount: payload.get("AMOUNT").unwrap().parse().unwrap(),
            timestamp: payload.get("TIMESTAMP").unwrap().parse().unwrap(),
            status: payload.get("STATUS").unwrap().parse().unwrap(),
            description: payload
                .get("DESCRIPTION")
                .unwrap()
                .trim()
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string(),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TX_ID: {:?}", self.tx_id)?;
        writeln!(f, "TX_TYPE: {:?}", self.tx_type)?;
        writeln!(f, "FROM_USER_ID: {:?}", self.from_user_id)?;
        writeln!(f, "TO_USER_ID: {:?}", self.to_user_id)?;
        writeln!(f, "AMOUNT: {:?}", self.amount)?;
        writeln!(f, "TIMESTAMP: {:?}", self.timestamp)?;
        writeln!(f, "STATUS: {:?}", self.status)?;
        writeln!(f, "DESCRIPTION: \"{}\"", self.description)?;

        Ok(())
    }
}

impl From<Vec<Record>> for TxtFormat {
    fn from(records: Vec<Record>) -> Self {
        TxtFormat { txt_rows: records }
    }
}

impl From<TxtFormat> for Vec<Record> {
    fn from(txt: TxtFormat) -> Self {
        txt.txt_rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Status, TxType};
    use std::io::Cursor;

    fn txt_record() -> Record {
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
    fn txt_write_then_read() -> Result<(), CustomError> {
        let rec = txt_record();

        let mut txt = TxtFormat {
            txt_rows: vec![rec.clone()],
        };

        let mut buf: Vec<u8> = Vec::new();

        {
            let mut cursor = Cursor::new(&mut buf);
            txt.write_to(&mut cursor)?;
        }

        let mut cursor = Cursor::new(&buf);

        let parsed = TxtFormat::from_read(&mut cursor)?;

        assert_eq!(parsed.txt_rows.len(), 1);
        assert_eq!(parsed.txt_rows[0], rec);

        Ok(())
    }
}
