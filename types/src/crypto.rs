use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use secp256k1::ecdsa::Signature;
use rand::RngCore; 
use crate::hash::sha256;

pub struct LibertyKeyPair {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl LibertyKeyPair {
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let mut rng = rand::rng();
        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Self {
            private_key: secret_key,
            public_key,
        }
    }


    pub fn from_private_key(hex_str: &str) -> Result<Self, secp256k1::Error> {
        let secret_key = SecretKey::from_slice(&hex::decode(hex_str).unwrap())?;
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok(Self { private_key: secret_key, public_key })
    }

    pub fn sign_message(&self, msg: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let hash = sha256(msg);
        let message = Message::from_slice(&hash).expect("32 bytes");
        secp.sign_ecdsa(&message, &self.private_key)
    }


    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    pub fn private_key_hex(&self) -> String {
        hex::encode(self.private_key.secret_bytes())
    }
pub fn verify_signature(pub_key: &PublicKey, msg: &[u8], sig: &Signature) -> bool {
    let secp = Secp256k1::verification_only();
    let hash = sha256(msg);
    let message = Message::from_slice(&hash).expect("Message must be 32 bytes");
    secp.verify_ecdsa(&message, sig, pub_key).is_ok()
}

}
pub fn verify_signature(public_key: &PublicKey, msg: &[u8], signature: &Signature) -> bool {
        let secp = Secp256k1::new();
        let hash = sha256(msg);
        let message = Message::from_slice(&hash).expect("32 bytes");
        secp.verify_ecdsa(&message, signature, public_key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation_and_signature() {
        let kp = LibertyKeyPair::generate();
        let message = b"libertycoin transaction";
        let signature = kp.sign_message(message);
        assert!(LibertyKeyPair::verify_signature(&kp.public_key, message, &signature));
    }
}
