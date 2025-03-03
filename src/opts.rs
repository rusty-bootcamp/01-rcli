use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{path::Path, str::FromStr};

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err("Invalid output format"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit_number: u8,
}

#[derive(Debug, Parser, Clone)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,
    #[arg(short, long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(short, long, default_value_t = true)]
    pub number: bool,
    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

fn verify_input_file(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File does not exist.")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    format.parse::<OutputFormat>()
}
