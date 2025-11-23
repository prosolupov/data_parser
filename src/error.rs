use csv::Error;
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

/// Файл с описанием ошибок

#[allow(dead_code)]
#[derive(Debug)]
pub enum CustomError {
    NotFound(PathBuf),
    Io(io::Error),
    InvalidData(String),
    PermissionDenied(PathBuf),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::NotFound(path) => writeln!(f, "Файл не найден: {}", path.display()),
            CustomError::Io(err) => write!(f, "Ошибка ввода-вывода: {}", err),
            CustomError::InvalidData(msg) => write!(f, "Неверные данные: {}", msg),
            CustomError::PermissionDenied(path) => {
                write!(f, "Нет прав для доступа к файлу: {}", path.display())
            }
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> Self {
        CustomError::Io(err)
    }
}

impl From<Error> for CustomError {
    fn from(err: Error) -> Self {
        CustomError::Io(err.into())
    }
}
