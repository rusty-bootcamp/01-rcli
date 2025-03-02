use csv::{Reader, ReaderBuilder, StringRecord};
use std::fs;

use crate::{CsvOpts, Player};

pub fn read_with_csv_builder(opts: CsvOpts) -> anyhow::Result<Vec<StringRecord>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(opts.delimiter as u8)
        .has_headers(opts.header)
        .from_path(opts.input)
        .expect("Failed to open CSV file");

    let records = rdr.records().collect::<Result<Vec<StringRecord>, _>>()?;

    Ok(records)
}

pub fn deserialize_csv(opts: CsvOpts) -> anyhow::Result<Vec<Player>> {
    let mut rdr = Reader::from_path(opts.input)?;

    let records = rdr
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, _>>()?;

    Ok(records)
}

pub fn serialize_record_to_json(records: Vec<StringRecord>, opts: CsvOpts) -> anyhow::Result<()> {
    let json_data: Vec<Vec<String>> = records
        .into_iter()
        .map(|record| record.into_iter().map(String::from).collect())
        .collect();

    let json_string = serde_json::to_string_pretty(&json_data)?;
    fs::write(format!("record_{}", opts.output), json_string)?;

    Ok(())
}

pub fn serialize_player_to_json(records: Vec<Player>, opts: CsvOpts) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(&records)?;
    fs::write(opts.output, json_string)?;

    Ok(())
}
