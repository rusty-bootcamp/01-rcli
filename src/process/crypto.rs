use std::{fs, path::Path};

use anyhow::{Ok, Result};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{DecryptOpts, EncryptFormat, EncryptOpts, GenPassOpts, GenerateKeyOpts, input_reader};

use super::process_passwd;

pub trait Encryptor {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error>;
}

pub trait Decryptor {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate_key() -> Result<Vec<Vec<u8>>, anyhow::Error>;
}

#[derive(Debug)]
pub struct Blake3 {
    key: [u8; 32],
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::load_key(key)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn load_key(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
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

impl KeyGenerator for Blake3 {
    fn generate_key() -> Result<Vec<Vec<u8>>, anyhow::Error> {
        let password_opts = GenPassOpts {
            length: 32,
            uppercase: true,
            lowercase: true,
            number: true,
            symbol: true,
        };

        let key = process_passwd(&password_opts)?;
        Ok(vec![key])
    }
}

pub struct Ed25519 {
    key: SigningKey,
}

impl KeyLoader for Ed25519 {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::load_key(key)
    }
}

impl Ed25519 {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn load_key(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
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

impl KeyGenerator for Ed25519 {
    fn generate_key() -> Result<Vec<Vec<u8>>, anyhow::Error> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();

        Ok(vec![sk, pk])
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::load_key(key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn load_key(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
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

pub fn process_generate(opts: &GenerateKeyOpts) -> Result<Vec<Vec<u8>>> {
    match opts.format {
        EncryptFormat::Blake3 => Blake3::generate_key(),
        EncryptFormat::Ed25519 => Ed25519::generate_key(),
    }
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

    #[test]
    fn test_ed25519_verify() -> Result<(), anyhow::Error> {
        let sk = Ed25519::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let content = "hello!";
        let sig = sk.encrypt(content)?;
        let bool = pk.decrypt(content, &sig)?;
        assert!(bool);

        Ok(())
    }
}
