use anyhow::{Ok, Result};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{DecryptOpts, EncryptFormat, EncryptOpts, input_reader};

pub trait Encryptor {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error>;
}

pub trait Decryptor {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error>;
}

#[derive(Debug)]
pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &str) -> Result<Self> {
        let key_bytes = key.as_bytes();
        if key_bytes.len() < 32 {
            return Err(anyhow::anyhow!("Key is too shot, need at least 32 bytes"));
        }

        let mut fixed_key = [0u8; 32];
        fixed_key.copy_from_slice(&key_bytes[..32]);
        Ok(Self::new(fixed_key))
    }
}

impl Encryptor for Blake3 {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error> {
        let encrypted = blake3::keyed_hash(&self.key, content.as_bytes())
            .as_bytes()
            .to_vec();
        Ok(encrypted)
    }
}

impl Decryptor for Blake3 {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error> {
        let decrypted = blake3::keyed_hash(&self.key, content.as_bytes())
            .as_bytes()
            .to_vec();

        Ok(decrypted == sig)
    }
}

pub struct Ed25519 {
    key: SigningKey,
}

impl Ed25519 {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &str) -> Result<Self, anyhow::Error> {
        let key = key.as_bytes();
        if key.len() < 32 {
            return Err(anyhow::anyhow!("Key is too shot, need at least 32 bytes"));
        }
        let mut fixed_key = [0u8; 32];
        fixed_key.copy_from_slice(&key[..32]);

        let key = SigningKey::from_bytes(&fixed_key);
        Ok(Self::new(key))
    }
}

impl Encryptor for Ed25519 {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error> {
        let encrypted = self.key.sign(content.as_bytes()).to_bytes().to_vec();
        Ok(encrypted)
    }
}

impl Decryptor for Ed25519 {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error> {
        let sig = Signature::from_bytes(sig.try_into()?);
        let decrypted = self.key.verify(content.as_bytes(), &sig);
        let bool = decrypted.is_ok();
        Ok(bool)
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &str) -> Result<Self, anyhow::Error> {
        let key = key.as_bytes();
        if key.len() < 32 {
            return Err(anyhow::anyhow!("Key is too shot, need at least 32 bytes"));
        }
        let mut fixed_key = [0u8; 32];
        fixed_key.copy_from_slice(&key[..32]);

        let key = VerifyingKey::from_bytes(&fixed_key)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

impl Decryptor for Ed25519Verifier {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error> {
        let sig = Signature::from_bytes(sig.try_into()?);
        let decrypted = self.key.verify(content.as_bytes(), &sig);
        let bool = decrypted.is_ok();
        Ok(bool)
    }
}

pub fn process_encrypt(opts: &EncryptOpts) -> Result<Vec<u8>, anyhow::Error> {
    let content = input_reader(&opts.input)?;
    let encrypted = match opts.format {
        EncryptFormat::Blake3 => {
            let signer = Blake3::try_new(&opts.key)?;
            signer.encrypt(&content)?
        }
        EncryptFormat::Ed25519 => {
            let signer = Ed25519::try_new(&opts.key)?;
            signer.encrypt(&content)?
        }
    };
    Ok(encrypted)
}

pub fn process_decrypt(opts: &DecryptOpts) -> Result<bool, anyhow::Error> {
    let content = input_reader(&opts.input)?;
    let sig = BASE64_URL_SAFE_NO_PAD.decode(&opts.sig)?;
    let decrypted = match opts.format {
        EncryptFormat::Blake3 => {
            let verifier = Blake3::try_new(&opts.key)?;
            verifier.decrypt(&content, &sig)?
        }
        EncryptFormat::Ed25519 => {
            let verifier = Ed25519Verifier::try_new(&opts.key)?;
            verifier.decrypt(&content, &sig)?
        }
    };
    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<(), anyhow::Error> {
        let key = "0123456789ABCDEFGHIJKLMNOPQRSTUV";
        let content = "test_content";
        let blake3 = Blake3::try_new(key)?;

        let sig = blake3.encrypt(content)?;
        let bool = blake3.decrypt(content, &sig)?;
        assert!(bool);
        Ok(())
    }
}
