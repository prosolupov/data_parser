#![warn(missing_docs)]

//! Библиотека предназначена для работы с тремя форматами файлов:
//! - **CSV**
//! - **TXT**
//! - **BIN** (кастомный бинарный формат)
//!
//! Основная идея библиотеки — приводить разные форматы к единой структуре данных
//! [`Record`](crate::models::Record), чтобы можно было:
//!
//! - читать файлы разных типов,
//! - конвертировать между форматами,
//! - сравнивать данные независимо от исходного формата,
//!
//! ## Основные возможности
//!
//! ### ✔ Чтение файлов
//! Функция [`file_reader`](crate::file_reader) определяет формат по параметру
//! и возвращает перечисление [`InputFormat`](crate::models::InputFormat),
//! содержащее распарсенные данные.
//!
//! Пример:
//!
//! ```rust
//! use parser::file_reader;
//! use parser::models::Format;
//!
//! // Перед использованием убедитесь, что файл находится в папке "static".
//! // Здесь пример создания тестового файла:
//! std::fs::create_dir_all("../static").unwrap();
//! std::fs::write(
//!     "../static/example.csv",
//!     "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
//!      1,DEPOSIT,10,20,100,123456789,SUCCESS,\"Test\"",
//! ).unwrap();
//!
//! let input = file_reader("example.csv", Format::Csv).unwrap();
//! let records = input.get_record();
//!
//! assert_eq!(records.len(), 1);
//! assert_eq!(records[0].tx_id, 1);
//! ```
//!
//! ### ✔ Конвертация между форматами
//!
//! Функция [`converter`](crate::converter) принимает:
//! - желаемый формат (`"csv"`, `"txt"`, `"bin"`)
//! - структуру [`InputFormat`], содержащую данные,
//!
//! и создаёт файл `static/output.<format>`.
//!
//! ```rust
//! use parser::{file_reader, converter};
//! use parser::models::Format;
//! use std::path::Path;
//!
//! std::fs::create_dir_all("../static").unwrap();
//! std::fs::write(
//!     "../static/sample.csv",
//!     "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
//!      1,DEPOSIT,1,2,50,42,SUCCESS,\"Hi\"",
//! ).unwrap();
//!
//! let input = file_reader("sample.csv", Format::Csv).unwrap();
//!
//! // Конвертация CSV → TXT
//! converter(Format::Txt, input);
//!
//! assert!(Path::new("../../static").join("output.txt").exists());
//! ```
//!
//! ## Модули
//!
//! - [`models`](crate::models) — структура [`Record`](crate::models::Record),
//!   перечисления [`TxType`](crate::models::TxType),
//!   [`Status`](crate::models::Status),
//!   [`InputFormat`](crate::models::InputFormat).
//!
//! - [`format`](crate::format) — реализации парсеров/писателей:
//!     - `CsvFormat`
//!     - `TxtFormat`
//!     - `BinFormat`
//!
//! - [`error`](crate::error) — перечисление [`CustomError`](crate::error::CustomError),
//!   объединяющее все возможные ошибки чтения и записи.
//!
//! ## Использование вместе с CLI
//!
//! В проекте есть две утилиты, использующие эту библиотеку:
//!
//! - **data_parser** — конвертирует файлы между форматами.
//! - **comparer** — сравнивает два файла построчно на уровне [`Record`].
//!
//! Обе утилиты работают одинаково благодаря единому внутреннему формату данных.
//!
//! ## Ошибки
//!
//! Все ошибки чтения, записи и парсинга объединены в [`CustomError`](crate::error::CustomError).
//! Через `thiserror` ошибки имеют удобный `Display` и автоматически
//! интегрируются с оператором `?`.
mod error;
mod format;

/// Файл содержит необходимые структуры данных
pub mod models;

use crate::format::DataFormat;
use crate::models::Record;
use error::CustomError;
use format::bin::BinFormat;
use format::csv::CsvFormat;
use format::txt::TxtFormat;
use models::Format;
use models::InputFormat;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Читает файл из директории `static/` и парсит его в один из форматов.
///
/// # Параметры
/// * `filename` — имя файла, без пути. Например: `"data.csv"`
/// * `file_type` — строка формата: `"csv"`, `"txt"`, `"bin"` (регистр не важен)
///
/// # Возвращает
/// * `Ok(InputFormat)` — обёртка над конкретным форматом (Csv/Txt/Bin),
///   из которой потом можно достать `Vec<Record>` через `get_record()`
/// * `Err(CustomError)` — если файл не удалось открыть/прочитать/распарсить.
///

pub fn file_reader(filename: &str, file_type: Format) -> Result<InputFormat, CustomError> {
    let path: PathBuf = Path::new("../../static").join(filename);
    let mut file: File = File::open(path)?;

    let file_format = if file_type == Format::Csv {
        InputFormat::Csv(CsvFormat::from_read(&mut file)?)
    } else if file_type == Format::Txt {
        InputFormat::Txt(TxtFormat::from_read(&mut file)?)
    } else if file_type == Format::Bin {
        InputFormat::Bin(BinFormat::from_read(&mut file)?)
    } else {
        return Err(CustomError::InvalidEnum(format!(
            "Неверный формат: {:?}",
            file_type
        )));
    };
    Ok(file_format)
}

/// Конвертирует уже прочитанные данные (`InputFormat`) в указанный формат
/// и записывает результат в файл `static/output.<type_output>`.
///
/// # Параметры
/// * `type_output` — целевой формат: `"csv"`, `"txt"`, `"bin"`
/// * `input_format` — исходные данные (Csv/Txt/Bin), уже распарсенные
///
/// # Поведение
/// * Все данные сначала приводятся к `Vec<Record>` через `get_record()`
/// * Затем создаётся соответствующий формат (`CsvFormat`, `TxtFormat`, `BinFormat`)
///   и вызывается его `write_to()`
/// * Файл всегда называется `output.<расширение>` и кладётся в папку `static/`.
///
/// # Возвращает
/// * `Ok(())` — если конвертация и запись прошли успешно
/// * `Err(CustomError)` — если возникли ошибки записи/конвертации
///
/// # Примечание
/// При неизвестном `type_output` функция *не возвращает ошибку*, а просто печатает
/// `"Input format not recognized"` и всё равно возвращает `Ok(())`.
pub fn converter(type_output: Format, input_format: InputFormat) -> Result<(), CustomError> {
    let records: Vec<Record> = input_format.get_record();
    let filename = match type_output {
        Format::Csv => format!("output.csv"),
        Format::Txt => format!("output.txt"),
        Format::Bin => format!("output.bin"),
    };
    let path: PathBuf = Path::new("../../static").join(filename);
    let mut file = File::create(path)?;

    match type_output {
        Format::Csv => {
            let mut out: CsvFormat = CsvFormat::from(records);
            out.write_to(&mut file)?;
        }
        Format::Txt => {
            let mut out: TxtFormat = TxtFormat::from(records);
            out.write_to(&mut file)?;
        }
        Format::Bin => {
            let mut out: BinFormat = BinFormat::from(records);
            out.write_to(&mut file)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Status, TxType};
    use std::fs;

    fn sample_record() -> Record {
        Record {
            tx_id: 1,
            tx_type: TxType::DEPOSIT,
            from_user_id: 10,
            to_user_id: 20,
            amount: 100,
            timestamp: 123456789,
            status: Status::SUCCESS,
            description: "Test record".to_string(),
        }
    }


    #[test]
    fn file_reader_reads_csv_round() -> Result<(), CustomError> {
        fs::create_dir_all("../static").unwrap();

        let records = vec![sample_record()];

        let path = Path::new("../static").join("test.csv");
        {
            let mut file = File::create(&path)?;
            let mut csv_format: CsvFormat = CsvFormat::from(records.clone());
            csv_format.write_to(&mut file)?;
        }

        let input_format = file_reader("../static/test.csv", Format::Csv)?;
        let parsed_records: Vec<Record> = input_format.get_record();

        assert_eq!(parsed_records, records);

        Ok(())
    }
    #[test]
    fn converter_creates_output_csv() -> Result<(), CustomError> {
        fs::create_dir_all("../static").unwrap();

        let records = vec![sample_record()];

        let out_path = Path::new("../../static").join("output.csv");
        let _ = fs::remove_file(&out_path);

        let csv_format: CsvFormat = CsvFormat::from(records.clone());
        let input = InputFormat::Csv(csv_format);

        converter(Format::Csv, input)?;

        assert!(out_path.exists());

        let metadata = fs::metadata(&out_path)?;
        assert!(metadata.len() > 0, "output.csv пустой");

        Ok(())
    }

    #[test]
    fn file_reader_nonexistent_file() {
        let res = file_reader("static/no_such_file.csv", Format::Csv);

        assert!(res.is_err());

        match res.unwrap_err() {
            CustomError::Io(_) => {}
            other => panic!("Expected Io error, got {:?}", other),
        }
    }
}
