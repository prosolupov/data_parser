mod error;
mod format;
mod models;

use std::ffi;
use crate::format::DataFormat;
use crate::error::CustomError;
use crate::format::bin::BinFormat;
use crate::format::csv::CsvFormat;
use crate::format::txt::TxtFormat;
use crate::models::{CliCommand, convert_format, InputFormat};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
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
    let mut file: File = File::open(path).unwrap();

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

fn converter(type_output: &str, input_format: InputFormat) -> Result<File, CustomError> {
    let path = Path::new("static").join(type_output);
    let mut file: File = File::open(path)?;
    let file_format: CsvFormat = DataFormat::from_read(&mut file)?;
    Ok(file)
}


fn main() {
    let params: CliCommand = CliCommand::parse();
    let data_file = file_reader(&params.input, &params.output_format);

}
