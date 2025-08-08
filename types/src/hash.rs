use sha2::{Sha256, Digest};

/// Returns a 32-byte SHA-256 hash of the given message.
pub fn sha256(message: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let input = b"libertycoin";
        let hash = sha256(input);
        assert_eq!(hash.len(), 32);
    }
}

