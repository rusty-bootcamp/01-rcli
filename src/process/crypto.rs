use anyhow::{Ok, Result};

use crate::{DecryptOpts, EncryptFormat, EncryptOpts, input_reader};

trait Encryptor {
    fn encrypt(&self, content: &str) -> Result<Vec<u8>, anyhow::Error>;
}

#[allow(dead_code)]
trait Decryptor {
    fn decrypt(&self, content: &str, sig: &[u8]) -> Result<bool, anyhow::Error>;
}

#[allow(dead_code)]
struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
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

/*
struct Ed25519 {
    key: [u8; 32],
}

impl Ed25519 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}*/

pub fn process_encrypt(opts: &EncryptOpts) -> Result<Vec<u8>, anyhow::Error> {
    let content = input_reader(&opts.input)?;
    let encrypted = match opts.format {
        EncryptFormat::Blake3 => {
            let key = opts.key.as_bytes().to_vec();
            let key = &key[..32];
            let encryptor = Blake3::new(key.try_into()?);
            encryptor.encrypt(&content)?
        }
        EncryptFormat::Ed25519 => todo!(),
    };
    Ok(encrypted)
}

pub fn process_decrypt(_opts: &DecryptOpts) -> Result<Vec<u8>, anyhow::Error> {
    let decrypted = Vec::new();
    Ok(decrypted)
}
