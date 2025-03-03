use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::Value;
use std::fs;

use crate::{CsvOpts, Player};

pub fn read_with_csv_builder(opts: CsvOpts) -> anyhow::Result<Vec<Value>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(opts.delimiter as u8)
        .has_headers(opts.header)
        .from_path(opts.input)
        .expect("Failed to open CSV file");

    let headers = rdr.headers()?.clone();
    let records = rdr.records().collect::<Result<Vec<StringRecord>, _>>()?;
    let records = records
        .iter()
        .map(|record| headers.iter().zip(record.iter()).collect::<Value>())
        .collect::<Vec<Value>>();

    Ok(records)
}

pub fn deserialize_csv(opts: CsvOpts) -> anyhow::Result<Vec<Player>> {
    let mut rdr = Reader::from_path(opts.input)?;

    let records = rdr
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, _>>()?;

    Ok(records)
}

pub fn serialize_record(records: Vec<Value>, opts: CsvOpts) -> anyhow::Result<()> {
    let content = match opts.format {
        crate::OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        crate::OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        crate::OutputFormat::Toml => {
            #[derive(serde::Serialize)]
            struct Record {
                item: Vec<Value>,
            }
            let record = Record { item: records };
            toml::to_string(&record)?
        }
    };

    fs::write(format!("record_{}", opts.output), content)?;

    Ok(())
}

pub fn serialize_player(records: Vec<Player>, opts: CsvOpts) -> anyhow::Result<()> {
    let content = match opts.format {
        crate::OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        crate::OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        crate::OutputFormat::Toml => {
            #[derive(serde::Serialize)]
            struct Record {
                item: Vec<Player>,
            }
            let record = Record { item: records };
            toml::to_string(&record)?
        }
    };

    fs::write(opts.output, content)?;

    Ok(())
}
