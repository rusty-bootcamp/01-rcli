use std::{fs, path::Path};

use anyhow::{Ok, Result};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{DecryptOpts, EncryptFormat, EncryptOpts, input_reader};

pub trait Encryptor {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

#[allow(dead_code)]
pub trait Decryptor {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error>;
}

#[allow(dead_code)]
pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Encryptor for Blake3 {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error> {
        let encrypted = blake3::hash(content.as_bytes()).as_bytes().to_vec();
        Ok(encrypted)
    }
}

impl Decryptor for Blake3 {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error> {
        let decrypted = blake3::hash(content.as_bytes()).as_bytes().to_vec();
        let bool = decrypted == sig;
        Ok(bool)
    }
}

pub struct Ed25519 {
    key: SigningKey,
}

impl Ed25519 {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self, anyhow::Error> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Ed25519 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
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

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self, anyhow::Error> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
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
            let signer = Blake3::load(&opts.key)?;
            signer.encrypt(&content)?
        }
        EncryptFormat::Ed25519 => {
            let signer = Ed25519::load(&opts.key)?;
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
            let verifier = Blake3::load(&opts.key)?;
            verifier.decrypt(&content, &sig)?
        }
        EncryptFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(&opts.key)?;
            verifier.decrypt(&content, &sig)?
        }
    };
    Ok(decrypted)
}
