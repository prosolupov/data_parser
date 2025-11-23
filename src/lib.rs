mod error;
mod format;
pub mod models;

use crate::format::DataFormat;
use crate::models::Record;
use error::CustomError;
use format::bin::BinFormat;
use format::csv::CsvFormat;
use format::txt::TxtFormat;
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
/// # Паника
/// Функция **паникнёт**, если `file_type` не равен ни `"csv"`, `"txt"`, `"bin"`.

pub fn file_reader(filename: &str, file_type: &str) -> Result<InputFormat, CustomError> {
    let path: PathBuf = Path::new("static").join(filename);
    let mut file: File = File::open(path)?;

    let file_format = if file_type.to_lowercase() == "csv" {
        InputFormat::Csv(CsvFormat::from_read(&mut file)?)
    } else if file_type.to_lowercase() == "txt" {
        InputFormat::Txt(TxtFormat::from_read(&mut file)?)
    } else if file_type.to_lowercase() == "bin" {
        InputFormat::Bin(BinFormat::from_read(&mut file)?)
    } else {
        panic!("Input format not recognized");
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
pub fn converter(type_output: &str, input_format: InputFormat) -> Result<(), CustomError> {
    let records: Vec<Record> = input_format.get_record();
    let filename = format!("output.{}", type_output);
    let path: PathBuf = Path::new("static").join(filename);
    let mut file = File::create(path)?;

    match type_output.to_lowercase().as_str() {
        "csv" => {
            let mut out: CsvFormat = CsvFormat::from(records);
            out.write_to(&mut file)?;
        }
        "txt" => {
            let mut out: TxtFormat = TxtFormat::from(records);
            out.write_to(&mut file)?;
        }
        "bin" => {
            let mut out: BinFormat = BinFormat::from(records);
            out.write_to(&mut file)?;
        }
        _ => {
            println!("Input format not recognized");
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
    fn file_reader_reads_csv_roundtrip() -> Result<(), CustomError> {
        fs::create_dir_all("static").unwrap();

        let records = vec![sample_record()];

        let path = Path::new("static").join("test.csv");
        {
            let mut file = File::create(&path)?;
            let mut csv_format: CsvFormat = CsvFormat::from(records.clone());
            csv_format.write_to(&mut file)?;
        }

        let input_format = file_reader("test.csv", "csv")?;
        let parsed_records: Vec<Record> = input_format.get_record();

        assert_eq!(parsed_records, records);

        Ok(())
    }
    #[test]
    fn converter_creates_output_csv() -> Result<(), CustomError> {
        fs::create_dir_all("static").unwrap();

        let records = vec![sample_record()];

        let out_path = Path::new("static").join("output.csv");
        let _ = fs::remove_file(&out_path);

        let csv_format: CsvFormat = CsvFormat::from(records.clone());
        let input = InputFormat::Csv(csv_format);

        converter("csv", input)?;

        assert!(out_path.exists());

        let metadata = fs::metadata(&out_path)?;
        assert!(metadata.len() > 0, "output.csv пустой");

        Ok(())
    }
}
