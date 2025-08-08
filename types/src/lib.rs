use serde::{Serialize, Deserialize};
pub mod hash;
pub mod crypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>, // Typically the pubkey hash or address
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: Vec<u8>,
    pub vout: usize,
}
