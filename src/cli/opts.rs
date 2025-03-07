use clap::Parser;
use enum_dispatch::enum_dispatch;

use super::{Base64Subcommand, CryptoSubcommand, CsvOpts, GenPassOpts, HttpSubCommand};

#[derive(Debug, Parser, Clone)]
#[enum_dispatch(CmdExecutor)]
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
