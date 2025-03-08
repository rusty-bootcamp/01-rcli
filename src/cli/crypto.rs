use std::{fs, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::{
    CmdExecutor, input_reader, process_decrypt, process_encrypt, process_generate, verify_path,
};

#[derive(Debug, Clone)]
pub enum EncryptFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<EncryptFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for EncryptFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(EncryptFormat::Blake3),
            "ed25519" => Ok(EncryptFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub enum CryptoSubcommand {
    #[command(about = "Encrypt a message with a private/shared key.")]
    Encrypt(EncryptOpts),
    #[command(about = "Decrypt a message with a private/shared key.")]
    Decrypt(DecryptOpts),
    #[command(about = "Generate a new key")]
    Generate(GenerateKeyOpts),
}

#[derive(Debug, Clone, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = input_reader)]
    pub key: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: EncryptFormat,
}

#[derive(Debug, Clone, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = input_reader, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = input_reader)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: EncryptFormat,
}

#[derive(Debug, Clone, Parser)]
pub struct GenerateKeyOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: EncryptFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for CryptoSubcommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            CryptoSubcommand::Encrypt(opts) => {
                let encrypted = process_encrypt(&opts.input, &opts.format, &opts.key)?;
                println!("{:?}", encrypted);
            }
            CryptoSubcommand::Decrypt(opts) => {
                let decrypted = process_decrypt(&opts.input, &opts.format, &opts.key, &opts.sig)?;
                println!("{:?}", decrypted);
            }
            CryptoSubcommand::Generate(opts) => {
                let key = process_generate(&opts.format)?;
                match opts.format {
                    EncryptFormat::Blake3 => {
                        let path = opts.output.join("blake3.txt");
                        fs::write(path, &key[0])?;
                    }
                    EncryptFormat::Ed25519 => {
                        let path = &opts.output;
                        fs::write(path.join("ed25519.sk"), &key[0])?;
                        fs::write(path.join("ed25519.pk"), &key[1])?;
                    }
                }
                println!("{:?}", key);
            }
        }
        Ok(())
    }
}
