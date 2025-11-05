use crate::format::Format;
use crate::models::Record;

#[derive(Debug)]
pub struct CsvFormat {
    pub csv_rows: Vec<Record>,
}

impl Format for CsvFormat {
    // Парсит из любого источника, реализующего трейт Read
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_reader(r);

        let mut csv_rows: Vec<Record> = Vec::new();

        for record in rdr.deserialize() {
            let record: Record = record?;
            csv_rows.push(record);
        }

        Ok(Self { csv_rows })
    }

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to<W: std::io::Write>(
        &mut self,
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_writer = csv::Writer::from_writer(writer);
        for row in &self.csv_rows {
            csv_writer.serialize(row)?;
        }
        csv_writer.flush()?;
        Ok(())
    }
}
