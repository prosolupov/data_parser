use csv::Error as CsvError;
use std::io;
use thiserror::Error;

/// Файл с описанием ошибок

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Ошибка ввода-вывода: {0}")]
    Io(#[from] io::Error),

    #[error("Ошибка чтения CSV: {0}")]
    Csv(#[from] CsvError),

    #[error("Неверные данные: {0}")]
    InvalidData(String),
}
