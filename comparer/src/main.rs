use clap::Parser;
use parser::file_reader;
use parser::models::{Format, InputFormat, Record};

/// CLI-команда `comparer___`
///
/// Позволяет сравнить два файла разных форматов (TXT, CSV, BIN),
/// построчно и поэлементно сравнив содержимое `Record`.
///
/// # Пример использования:
/// ```bash
/// comparer___ --file1 a.bin --format1 bin --file2 b.csv --format2 csv
/// ```
///
/// # Опции:
/// - `--file1` — путь к первому файлу
/// - `--format1` — его формат (`csv`, `txt`, `bin`)
/// - `--file2` — путь ко второму файлу
/// - `--format2` — его формат
///
/// Файлы читаются через `file_reader`, который возвращает `InputFormat`.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliCommandComparer {
    #[arg(long)]
    pub file1: String,
    #[arg(long)]
    pub format1: Format,
    #[arg(long)]
    pub file2: String,
    #[arg(long)]
    pub format2: Format,
}

fn check_file(file_one: InputFormat, file_two: InputFormat) -> Option<Record> {
    let file1 = InputFormat::get_record(file_one);
    let file2 = InputFormat::get_record(file_two);

    let len = file1.len().max(file2.len());

    for i in 0..len {
        if file1[i] != file2[i] {
            return Some(file1[i].clone());
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params: CliCommandComparer = CliCommandComparer::parse();
    let file_one = file_reader(&params.file1, params.format1);
    let file_two = file_reader(&params.file2, params.format2);

    let file1 = match file_one {
        Ok(data_file) => data_file,
        Err(e) => return Err(Box::new(e)),
    };

    let file2 = match file_two {
        Ok(data_file) => data_file,
        Err(e) => return Err(Box::new(e)),
    };

    let result = check_file(file1, file2);

    match result {
        Some(record) => println!("{}", record),
        None => println!(
            "The transaction records in '{}' and '{}' are identical.",
            &params.file1, &params.file2
        ),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    #[test]
    fn check_cli_comparer() {
        let args = CliCommandComparer::parse_from([
            "comparer",
            "--file1",
            "a.bin",
            "--format1",
            "bin",
            "--file2",
            "b.csv",
            "--format2",
            "csv",
        ]);

        assert_eq!(args.file1, "a.bin");
        assert_eq!(args.format1, Format::Bin);
        assert_eq!(args.file2, "b.csv");
        assert_eq!(args.format2, Format::Csv);
    }
}
