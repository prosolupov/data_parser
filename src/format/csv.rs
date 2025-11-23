use crate::error::CustomError;
use crate::format::DataFormat;
use crate::models::Record;

/// CSV формат
/// Чтение и создание csv формата

#[derive(Debug)]
pub struct CsvFormat {
    pub csv_rows: Vec<Record>,
}

impl DataFormat for CsvFormat {
    // Парсит из любого источника, реализующего трейт Read
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, CustomError> {
        let mut rdr = csv::Reader::from_reader(r);

        let mut csv_rows: Vec<Record> = Vec::new();

        for record in rdr.deserialize() {
            let record: Record = record?;
            csv_rows.push(record);
        }

        Ok(Self { csv_rows })
    }

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), CustomError> {
        let mut csv_writer = csv::Writer::from_writer(writer);
        for row in &self.csv_rows {
            let _ = csv_writer.serialize(row);
        }
        csv_writer.flush()?;
        Ok(())
    }
}

impl From<CsvFormat> for Vec<Record> {
    fn from(format: CsvFormat) -> Self {
        format.csv_rows
    }
}

impl From<Vec<Record>> for CsvFormat {
    fn from(records: Vec<Record>) -> Self {
        CsvFormat { csv_rows: records }
    }
}
