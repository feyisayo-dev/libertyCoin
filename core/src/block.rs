use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use types::{TransactionInput, TransactionOutput};
use types::hash::sha256;
use crate::utxo_set::UTXOSet;
use crate::transaction::verify_transaction;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
    if transactions.is_empty() {
        return sha256(&[]);
    }

    let mut hashes: Vec<[u8; 32]> = transactions
        .iter()
        .map(|tx| {
            let encoded = bincode::serialize(tx).unwrap();
            sha256(&encoded)
        })
        .collect();

    while hashes.len() > 1 {
        let mut new_hashes = vec![];
        for i in (0..hashes.len()).step_by(2) {
            let left = hashes[i];
            let right = if i + 1 < hashes.len() {
                hashes[i + 1]
            } else {
                left // Duplicate last
            };
            let mut combined = vec![];
            combined.extend_from_slice(&left);
            combined.extend_from_slice(&right);
            new_hashes.push(sha256(&combined));
        }
        hashes = new_hashes;
    }

    hashes[0]
}

impl BlockHeader {
    pub fn hash(&self) -> [u8; 32] {
        let encoded = bincode::serialize(self).unwrap(); // Simple for now
        sha256(&encoded)
    }
}

pub fn create_genesis_transaction() -> Transaction {
    Transaction {
        inputs: vec![TransactionInput {
           txid: vec![0u8; 32],
           vout: 0,
           signature: vec![],
           pub_key: vec![],
        }],
        outputs: vec![TransactionOutput {
            value: 50 * 100_000_000,
            script_pubkey: vec![],
        }],
        timestamp: 1754192000000,
    }
}


impl Block {
    pub fn verify_block(&self, utxos: &UTXOSet) -> bool {
        // 1. Merkle root check
        let computed_merkle = calculate_merkle_root(&self.transactions);
        if self.header.merkle_root != computed_merkle {
            return false;
        }

        // 2. Verify each transaction
        for tx in &self.transactions {
            if !verify_transaction(tx, utxos) {
                return false;
            }
        }

        true
    }
}


