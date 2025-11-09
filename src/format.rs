pub mod bin;
pub mod csv;
pub mod txt;

pub trait Format
where
    Self: Sized,
{
    // Парсит из любого источника, реализующего трейт Read
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>>;

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to<W: std::io::Write>(
        &mut self,
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
