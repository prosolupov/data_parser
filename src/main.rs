mod error;
mod format;
mod models;
use data_parser::{converter, file_reader};

use crate::format::DataFormat;
use clap::Parser;

/// CLI-интерфейс для утилиты **data_parser**.
///
/// Позволяет конвертировать файлы между форматами:
///
/// - `csv`  ←→  `txt`
/// - `csv`  ←→  `bin`
/// - `txt`  ←→  `bin`
///
/// Формат входных файлов должен соответствовать одному из поддерживаемых
/// форматов, а выходной формат задаётся явно.
///
/// # Пример использования
///
/// ```bash
/// data_parser -i transactions.csv -f csv -o bin
/// ```
///
/// Это прочитает файл `static/transactions.csv`,
/// распарсит его как CSV,
/// и создаст файл `static/output.bin` в бинарном формате.
///
/// # Аргументы:
///
/// * `--input`, `-i` — имя входного файла (только имя, без пути)
/// * `--input-format`, `-f` — формат входного файла: `csv`, `txt`, `bin`
/// * `--output-format`, `-o` — формат выходного файла: `csv`, `txt`, `bin`
///
/// Все файлы читаются и создаются внутри директории `static/`.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliCommandDataParser {
    #[arg(short = 'i', long)]
    pub input: String,

    #[arg(short = 'f', long)]
    pub input_format: String,

    #[arg(short = 'o', long)]
    pub output_format: String,
}
fn main() {
    let params: CliCommandDataParser = CliCommandDataParser::parse();
    let data_file = file_reader(&params.input, &params.input_format);

    let data = match data_file {
        Ok(data_file) => data_file,
        Err(e) => return println!("{}", e),
    };

    let _ = converter(&params.output_format, data);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_cli_data_parser() {
        let args = CliCommandDataParser::parse_from([
            "data_parser",
            "--input",
            "a.bin",
            "--input-format",
            "bin",
            "--output-format",
            "csv",
        ]);

        assert_eq!(args.input, "a.bin");
        assert_eq!(args.input_format, "bin");
        assert_eq!(args.output_format, "csv");
    }
}
