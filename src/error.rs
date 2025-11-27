use csv::Error as CsvError;
use std::io::Error as IoError;
use thiserror::Error;

/// Файл с описанием ошибок

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Ошибка ввода-вывода: {0}")]
    Io(#[from] IoError),

    #[error("Ошибка чтения CSV: {0}")]
    Csv(#[from] CsvError),

    #[error("Неверные данные: {0}")]
    InvalidData(String),

    #[error("Отсутствует поле: {0}")]
    MissingField(String),

    #[error("Поле {0} должно быть числом")]
    InvalidNumber(String),

    #[error("{0}")]
    InvalidEnum(String),
}
