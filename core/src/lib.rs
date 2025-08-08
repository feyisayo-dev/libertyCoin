pub mod block;
pub mod transaction;
pub use types::TransactionOutput;
pub mod utxo;
pub mod utxo_set;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use types::crypto::{LibertyKeyPair, verify_signature};

    #[test]
    fn test_sign_and_verify() {
        let keys = LibertyKeyPair::generate();
        let message = b"test message";
        let sig = keys.sign_message(message);

        assert!(
            verify_signature(&keys.public_key, message, &sig),
            "Signature should be valid"
        );
    }
}

