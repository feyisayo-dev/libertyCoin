use core::{
    transaction::Transaction,
    crypto::{generate_keypair, sign},
    hash::sha256,
};
use types::{TransactionInput, TransactionOutput};
use secp256k1::PublicKey;

#[test]
fn test_utxo_transaction_signature_verification() {
    // Generate keypair for the sender
    let (sk, pk) = generate_keypair();

    // Fake previous transaction ID (normally from a real tx)
    let prev_txid = sha256(b"dummy prev tx");

    // Output index from the previous transaction
    let vout_index = 0;

    // Create a new output (e.g., recipient of 50 units)
    let output = TransactionOutput {
        value: 50,
        public_key: pk, // we can reuse same key for simplicity
    };

    // Construct transaction
    let tx = Transaction::new(
        vec![(prev_txid.to_vec(), vout_index, &sk)],
        vec![output.clone()],
    );

    // Verify that the transaction is valid
    assert!(tx.is_valid(), "Transaction signature failed verification");
}
