#[cfg(test)]
mod tests {
    use crate::block::{genesis_block};
    #[test]
    fn test_genesis_block_hash() {
        let block = genesis_block();
        let hash = block.header.hash();
        println!("Genesis block hash: {:x?}", hash);
    }
}
