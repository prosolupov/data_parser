use crate::error::CustomError;

pub mod bin;
pub mod csv;
pub mod txt;

/// Родительский trait для всех форматов файлов
pub trait DataFormat
where
    Self: Sized,
{
    // Парсит из любого источника, реализующего трейт Read
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, CustomError>;

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), CustomError>;
}
