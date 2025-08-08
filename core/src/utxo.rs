use types::TransactionOutput;

#[derive(Debug, Clone)]
pub struct UTXO {
    pub txid: [u8; 32],
    pub index: u32,
    pub output: TransactionOutput,
}
