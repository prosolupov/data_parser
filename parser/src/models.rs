use crate::format::bin::BinFormat;
use crate::format::csv::CsvFormat;
use crate::format::txt::TxtFormat;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

///Перечисление принимаемых форматов
#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Format {
    ///Формат CSV
    #[value(name = "csv")]
    Csv,
    ///Формат Txt
    #[value(name = "txt")]
    Txt,
    ///Формат Bin
    #[value(name = "bin")]
    Bin,
}

/// Перечисление, представляющее входной формат данных.
///
/// При чтении файла (CSV, TXT или BIN) конкретный формат парсит данные и
/// конвертируется в один из вариантов этого enum’а.
///
/// Это позволяет единообразно работать с разными форматами, затем вызывая
/// `get_record()` чтобы получить `Vec<Record>`.
#[derive(Debug)]
pub enum InputFormat {
    ///Входной формат Csv
    Csv(CsvFormat),
    ///Входной формат Txt
    Txt(TxtFormat),
    ///Входной формат Bin
    Bin(BinFormat),
}

impl InputFormat {
    ///Функция для получения вектора из InputFormat
    pub fn get_record(self) -> Vec<Record> {
        match self {
            InputFormat::Csv(csv) => csv.into(),
            InputFormat::Txt(txt) => txt.into(),
            InputFormat::Bin(bin) => bin.into(),
        }
    }
}

/// Тип транзакции.
///
/// Используется в файлах TXT, CSV и BIN.
#[derive(EnumString, Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[strum(serialize_all = "UPPERCASE")]
///Перечисление типов транзакций
pub enum TxType {
    ///Тип DEPOSIT
    DEPOSIT,
    ///Тип TRANSFER
    TRANSFER,
    ///Тип WITHDRAWAL
    WITHDRAWAL,
}

impl From<u8> for TxType {
    fn from(item: u8) -> Self {
        match item {
            0 => TxType::DEPOSIT,
            1 => TxType::TRANSFER,
            2 => TxType::WITHDRAWAL,
            _ => panic!(),
        }
    }
}

/// Статус транзакции.
#[derive(EnumString, Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[strum(serialize_all = "UPPERCASE")]
///Перечисление состояний транзакций
pub enum Status {
    ///Состояние SUCCESS
    SUCCESS,
    ///Состояние FAILURE
    FAILURE,
    ///Состояние PENDING
    PENDING,
}

impl From<u8> for Status {
    /// Позволяет распарсить Status из байтов.
    fn from(item: u8) -> Self {
        match item {
            0 => Status::SUCCESS,
            1 => Status::FAILURE,
            2 => Status::PENDING,
            _ => panic!(),
        }
    }
}

/// Структура файлов
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Record {
    ///неотрицательное целое число, идентифицирующее транзакцию
    pub tx_id: u64,
    ///тип транзакции: `DEPOSIT`, `TRANSFER`, или `WITHDRAWAL`
    pub tx_type: TxType,
    ///неотрицательное целое число, идентифицирующее отправитель счета
    pub from_user_id: u64,
    ///неотрицательное целое число, идентифицирующее получателя счета
    pub to_user_id: u64,
    ///неотрицательное целое число, представляющее сумму в наименьшей единице валюты
    pub amount: u64,
    ///Unix epoch timestamp в миллисекундах
    pub timestamp: u64,
    ///состояние транзакции: `SUCCESS`, `FAILURE`, или `PENDING`
    pub status: Status,
    ///произвольное текстовое описание, UTF-8 в двойныхкавычках
    pub description: String,
}

// pub fn convert_format<A, B>(a: A) -> B
// where
//     A: Into<Vec<Record>>,
//     B: From<Vec<Record>>,
// {
//     B::from(a.into())
// }
