use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CsvRow {
    tx_id: u64,
    tx_type: String,
    from_user_id: u64,
    to_user_id: u64,
    amount: f64,
    timestamp: String,
    status: String,
    description: String,
}

#[derive(Debug)]
pub struct CsvFormat {
    pub csv_rows: Vec<CsvRow>,
}

impl CsvFormat {
    // Парсит из любого источника, реализующего трейт Read
    pub fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_reader(r);

        let mut csv_rows: Vec<CsvRow> = Vec::new();

        for record in rdr.deserialize() {
            let record: CsvRow = record?;
            csv_rows.push(record);
        }

        Ok(Self { csv_rows })
    }

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    pub fn write_to<W: std::io::Write>(
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
