use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    tx_id: u64,
    tx_type: TxType,
    from_user_id: u64,
    to_user_id: u64,
    amount: f64,
    timestamp: u64,
    status: Status,
    description: String,
}
