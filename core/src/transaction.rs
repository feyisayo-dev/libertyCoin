use types::hash::sha256;
use secp256k1::ecdsa::Signature;
use serde::{Serialize, Deserialize};
use crate::utxo_set::UTXOSet;
use types::{TransactionInput, TransactionOutput};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use types::crypto::verify_signature;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(inputs: Vec<(Vec<u8>, usize, &SecretKey)>, outputs: Vec<TransactionOutput>) -> Self {
        let mut tx_inputs = vec![];

        for (txid, vout, sk) in inputs {
            let msg = format!("{:?}{:?}", txid, vout);
            let hash = sha256(msg.as_bytes());
            let message = secp256k1::Message::from_slice(&hash).unwrap();
            let secp = Secp256k1::new();
            let signature = secp.sign_ecdsa(&message, sk);
            let pub_key = PublicKey::from_secret_key(&secp, sk);

            tx_inputs.push(TransactionInput {
                txid,
                vout,
                signature: signature.serialize_der().to_vec(),
                pub_key: pub_key.serialize().to_vec(),

            });
        }
        Self {
            inputs: tx_inputs,
            outputs,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as u128,
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let serialized = bincode::serialize(self).expect("Failed to serialize transaction");
        sha256(&serialized).to_vec()
    }


    pub fn is_valid(&self) -> bool {
        for input in &self.inputs {
            let sig = &input.signature;
            let pk = &input.pub_key;
            let msg = self.hash();
            let pub_key = PublicKey::from_slice(pk).unwrap();
            let signature = Signature::from_der(sig).unwrap();
            if !verify_signature(&pub_key, &msg, &signature) {
                return false;
            }
        }
        true
    }
}

pub fn verify_transaction(tx: &Transaction, _utxos: &UTXOSet) -> bool {
    tx.is_valid()
}