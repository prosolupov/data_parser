use clap::Parser;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

impl FromStr for TxType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TxType::DEPOSIT),
            "TRANSFER" => Ok(TxType::TRANSFER),
            "WITHDRAWAL" => Ok(TxType::WITHDRAWAL),
            _ => Err(format!("Нет такого tx type {}", s)),
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

impl FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Status::SUCCESS),
            "FAILURE" => Ok(Status::FAILURE),
            "PENDING" => Ok(Status::PENDING),
            _ => Err(format!("Нет такого статуса {}", s)),
        }
    }
}

impl From<u8> for Status {
    fn from(item: u8) -> Self {
        match item {
            0 => Status::SUCCESS,
            1 => Status::FAILURE,
            2 => Status::PENDING,
            _ => panic!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: Status,
    pub description: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliCommand {
    #[arg(short = 'i', long)]
    pub input: String,

    #[arg(short = 'f', long)]
    pub input_format: String,

    #[arg(short = 'o', long)]
    pub output_format: String,
}
