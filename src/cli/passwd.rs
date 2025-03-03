use clap::Parser;

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
