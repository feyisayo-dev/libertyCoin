use serde::{Serialize, Deserialize};
use types::{TransactionInput, TransactionOutput};
use types::hash::sha256;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub locktime: u32,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction {
            version: 1,
            inputs,
            outputs,
            locktime: 0,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        // Basic serialization for hashing
        let encoded = bincode::serialize(self).expect("Serialization failed");
        sha256(&encoded)
    }
}
