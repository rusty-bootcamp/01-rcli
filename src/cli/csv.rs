use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    CmdExecutor, deserialize_csv, read_with_csv_builder, serialize_player, serialize_record,
    verify_input_file,
};

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

fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    format.parse::<OutputFormat>()
}

impl CmdExecutor for CsvOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let records = read_with_csv_builder(&self.input, self.delimiter, self.header)?;
        serialize_record(records, self.format.clone(), &self.output)?;

        let palyers = deserialize_csv(&self.input)?;
        serialize_player(palyers, self.format.clone(), &self.output)?;
        Ok(())
    }
}
