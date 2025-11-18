mod error;
mod format;
mod models;

use crate::error::CustomError;
use crate::format::bin::BinFormat;
use crate::format::csv::CsvFormat;
use crate::format::txt::TxtFormat;
use crate::format::DataFormat;
use crate::models::{CliCommand, InputFormat, Record};
use clap::Parser;
use std::fs::File;
use std::path::{Path, PathBuf};

// fn reader_file(filename: String) -> Result<File, Box<dyn std::error::Error>> {
//     let path = Path::new("static").join(filename);
//     let mut file = File::open(path);
//     let file_open = match file {
//         Ok(mut file) => file,
//         Err(e) => {
//             return Err(Box::new(e));
//         }
//     };
//     Ok(file_open)
// }

fn file_reader(filename: &str, file_type: &str) -> Result<InputFormat, CustomError> {

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

fn converter(type_output: &str, input_format: InputFormat) -> Result<(), CustomError> {
    let records: Vec<Record> = input_format.get_record();
    let filename = format!("output.{}", type_output);
    let path: PathBuf = Path::new("static").join(filename);
    let mut file = File::create(path)?;

    match type_output.to_lowercase().as_str() {
        "csv" => {
            let mut out: CsvFormat = CsvFormat::from(records);
            out.write_to(&mut file)?;
        },
        "txt" => {
            let mut out: TxtFormat = TxtFormat::from(records);
            out.write_to(&mut file)?;
        },
        "bin" => {
            let mut out: BinFormat = BinFormat::from(records);
            out.write_to(&mut file)?;
        },
        _ => {
            println!("Input format not recognized");
        }
    }

    Ok(())
}


fn main() {
    let params: CliCommand = CliCommand::parse();
    let data_file = file_reader(&params.input, &params.input_format);

    let data = match data_file {
        Ok(data_file) => data_file,
        Err(e) => return println!("{}", e),
    };

    let _ = converter(&params.output_format, data);

}
