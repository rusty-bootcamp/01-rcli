use clap::Parser;

use super::{Base64Subcommand, CryptoSubcommand, CsvOpts, GenPassOpts, HttpSubCommand};
use crate::CmdExecutor;

#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode/decode base64")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Encrypt/decrypt data")]
    Crypto(CryptoSubcommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

impl CmdExecutor for SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(opts) => opts.execute().await,
            SubCommand::Crypto(opts) => opts.execute().await,
            SubCommand::Http(opts) => opts.execute().await,
        }
    }
}
