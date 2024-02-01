use ed25519_dalek::{PublicKey, Signature, Verifier};

#[derive(Debug)]
pub struct DiscordVerify {
    pub public_key: String,
    pub signature: String,
    pub timestamp: String,
}

impl DiscordVerify {
    pub fn verify_request(&self, body: &String) -> bool {
        let public_key = &hex::decode(&self.public_key)
            .and_then(|bytes| Ok(PublicKey::from_bytes(&bytes)))
            .unwrap()
            .unwrap();

        let signature = &hex::decode(&self.signature)
            .and_then(|byte| Ok(Signature::from_bytes(&byte)))
            .unwrap()
            .unwrap();

        let msg = format!("{}{}", self.timestamp, body);

        public_key.verify(msg.as_bytes(), signature).is_ok()
    }
}
