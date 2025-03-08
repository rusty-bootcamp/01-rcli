use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::Value;
use std::fs;

use crate::{OutputFormat, Player};

pub fn read_with_csv_builder(
    input: &str,
    delimiter: char,
    has_headers: bool,
) -> anyhow::Result<Vec<Value>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_path(input)
        .expect("Failed to open CSV file");

    let headers = rdr.headers()?.clone();
    let records = rdr.records().collect::<Result<Vec<StringRecord>, _>>()?;
    let records = records
        .iter()
        .map(|record| headers.iter().zip(record.iter()).collect::<Value>())
        .collect::<Vec<Value>>();

    Ok(records)
}

pub fn deserialize_csv(input: &str) -> anyhow::Result<Vec<Player>> {
    let mut rdr = Reader::from_path(input)?;

    let records = rdr
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, _>>()?;

    Ok(records)
}

pub fn serialize_record(
    records: Vec<Value>,
    format: OutputFormat,
    output: &str,
) -> anyhow::Result<()> {
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        OutputFormat::Toml => {
            #[derive(serde::Serialize)]
            struct Record {
                item: Vec<Value>,
            }
            let record = Record { item: records };
            toml::to_string(&record)?
        }
    };

    fs::write(format!("record_{}", output), content)?;

    Ok(())
}

pub fn serialize_player(
    records: Vec<Player>,
    format: OutputFormat,
    output: &str,
) -> anyhow::Result<()> {
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        OutputFormat::Toml => {
            #[derive(serde::Serialize)]
            struct Record {
                item: Vec<Player>,
            }
            let record = Record { item: records };
            toml::to_string(&record)?
        }
    };

    fs::write(output, content)?;

    Ok(())
}
