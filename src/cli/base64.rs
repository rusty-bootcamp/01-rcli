use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{CmdExecutor, process_decode, process_encode, verify_input_file};

use super::parse_base64_format;

#[derive(Debug, Clone, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),
}

#[derive(Debug, Clone, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Parser)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format.")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url",
        }
    }
}

impl CmdExecutor for EncodeOpts {
    async fn execute(&self) -> Result<(), anyhow::Error> {
        let encoded = process_encode(&self.input, &self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for DecodeOpts {
    async fn execute(&self) -> Result<(), anyhow::Error> {
        let encoded = process_decode(&self.input, &self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}
