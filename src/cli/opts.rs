use clap::Parser;

use super::{Base64Subcommand, CryptoSubcommand, CsvOpts, GenPassOpts, HttpSubCommand};

#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
    #[command(subcommand)]
    Crypto(CryptoSubcommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
