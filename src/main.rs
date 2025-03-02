use std::path::Path;

use clap::Parser;
use csv::{Reader, ReaderBuilder, StringRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

#[derive(Debug, Parser, Clone)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

#[derive(Debug, Parser, Clone)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

fn verify_input_file(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File does not exist.")
    }
}

fn read_with_csv_builder(opts: CsvOpts) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(opts.delimiter as u8)
        .has_headers(opts.header)
        .from_path(opts.input)
        .expect("Failed to open CSV file");

    let records = rdr.records().collect::<Result<Vec<StringRecord>, _>>()?;

    for record in records {
        println!("{:?}", record);
    }

    Ok(())
}

fn deserialize_csv(opts: CsvOpts) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(opts.input)?;

    let records = rdr
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, _>>()?;

    for player in records {
        println!("{:?}", player);
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            read_with_csv_builder(opts.clone())?;
            deserialize_csv(opts)?;
        }
    }

    Ok(())
}
