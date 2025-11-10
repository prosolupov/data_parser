mod format;
mod models;
use crate::format::Format;
use crate::format::bin::BinFormat;
use crate::format::csv::CsvFormat;
use crate::format::txt::TxtFormat;
use crate::models::{convert_format, CliCommand};
use clap::Parser;
use std::fs::File;

fn job_csv() -> Result<(), Box<dyn std::error::Error>> {
    //Чтение файла
    let mut file: File = File::open("static/csv_example.csv").unwrap();
    let csv = CsvFormat::from_read(&mut file);

    //Создание файла
    let mut new_file = File::create("static/wr.csv")?;
    // let mut csF: CsvFormat = csv1.unwrap();
    let mut f_csv: CsvFormat = csv?;
    f_csv.write_to(&mut new_file)
}

fn job_txt() -> Result<(), Box<dyn std::error::Error>> {
    //Чтение файла
    let mut file: File = File::open("static/txt_example.txt").unwrap();
    let txt = TxtFormat::from_read(&mut file);

    //Создание файла
    let mut new_file = File::create("static/wr.txt")?;
    let mut f_txt: TxtFormat = txt?;
    let _ = f_txt.write_to(&mut new_file);

    Ok(())
}

fn job_bin() -> Result<(), Box<dyn std::error::Error>> {
    //Чтение файла
    let mut file: File = File::open("static/bin_example.bin").unwrap();
    let f_bin = BinFormat::from_read(&mut file);

    //Создание файла
    let mut new_file = File::create("static/wr.bin")?;
    let mut f_bin: BinFormat = f_bin?;
    let _ = f_bin.write_to(&mut new_file);

    Ok(())
}

fn converter(filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::open("static/csv_example.csv").unwrap();
    let mut new_file = File::create("static/output.txt")?;
    let csv:CsvFormat = Format::from_read(&mut file)?;
    let mut txt: TxtFormat = convert_format(csv);
    let _ = txt.write_to(&mut new_file);

    Ok(())
}

fn main() {
    // job_csv().unwrap();
    // job_txt().unwrap();
    // job_bin().unwrap();

    let params: CliCommand = CliCommand::parse();
    converter(&params.input).expect("TODO: panic message");

    print!("{:?}", params);
}
