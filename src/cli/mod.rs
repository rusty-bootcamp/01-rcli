mod base64;
mod csv;
mod opts;
mod passwd;

use std::{fs::File, io::Read, path::Path};

pub use base64::*;
pub use csv::*;
pub use opts::*;
pub use passwd::*;

pub fn verify_input_file(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() || input == "-" {
        Ok(input.into())
    } else {
        Err("File does not exist.")
    }
}

pub fn parse_base64_format(format: &str) -> Result<Base64Format, &'static str> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "url" => Ok(Base64Format::UrlSafe),
        _ => Err("Invalid base64 format."),
    }
}

pub fn file_reader(input: &str) -> anyhow::Result<String> {
    let mut rdr: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = String::new();
    rdr.read_to_string(&mut buf)?;

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("nonexistent_file"),
            Err("File does not exist.")
        );
    }
}
