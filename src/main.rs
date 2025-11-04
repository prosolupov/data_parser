mod format;

use format::f_csv::CsvFormat;
use std::fs::File;

fn job_csv() -> Result<(), Box<dyn std::error::Error>> {
    //Чтение файла
    let mut file: File = File::open("static/csv_example.csv").unwrap();
    let csv = CsvFormat::from_read(&mut file);
    // let csv1 = CsvFormat::from_read(&mut file);
    // println!("{:?}", &csv.unwrap());

    //Создание файла
    let mut new_file = File::create("static/wr.csv")?;
    // let mut csF: CsvFormat = csv1.unwrap();
    let mut f_csv: CsvFormat = csv?;
    f_csv.write_to(&mut new_file)
}

fn main() {
    job_csv().unwrap();
}
