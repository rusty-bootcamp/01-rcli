use clap::Parser;

use crate::{CmdExecutor, process_passwd};

#[derive(Debug, Clone, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = false)]
    pub uppercase: bool,
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    pub lowercase: bool,
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub number: bool,
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let passwd = process_passwd(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", String::from_utf8(passwd)?);
        Ok(())
    }
}
