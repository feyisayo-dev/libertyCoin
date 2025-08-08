use std::collections::HashMap;
use types::{TransactionInput, TransactionOutput, OutPoint};
use crate::transaction::Transaction;

#[derive(Default, Debug, Clone)]
pub struct UTXOSet {
    pub utxos: HashMap<OutPoint, TransactionOutput>,
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet {
            utxos: HashMap::new(),
        }
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) {
        // Remove spent outputs
        for input in &tx.inputs {
            let outpoint = OutPoint {
                txid: input.txid.clone(),
                vout: input.vout,
            };
            self.utxos.remove(&outpoint);
        }

        // Add new outputs
        let _txid = tx.hash();
        for (index, output) in tx.outputs.iter().enumerate() {
        let outpoint = OutPoint {
            txid: tx.hash(),
            vout: index,
        };
        self.utxos.insert(outpoint, output.clone());
        }
    }

    pub fn is_unspent(&self, input: &TransactionInput) -> bool {
        let outpoint = OutPoint {
            txid: input.txid.clone(),
            vout: input.vout,
        };
        self.utxos.contains_key(&outpoint)
    }

    pub fn get_output(&self, outpoint: &OutPoint) -> Option<&TransactionOutput> {
        self.utxos.get(outpoint)
    }

    pub fn insert_output(&mut self, outpoint: OutPoint, output: TransactionOutput) {
        self.utxos.insert(outpoint, output);
    }

    pub fn remove_output(&mut self, outpoint: &OutPoint) {
        self.utxos.remove(outpoint);
    }

    pub fn verify_transaction(&self, tx: &Transaction) -> bool {
        tx.inputs.iter().all(|input| {
            let outpoint = OutPoint {
                txid: input.txid.clone(),
                vout: input.vout,
            };
            self.utxos.get(&outpoint).map_or(false, |output| {
                output.script_pubkey == input.pub_key
            })
        })
    }
}
