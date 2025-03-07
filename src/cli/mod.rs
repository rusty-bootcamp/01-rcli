mod base64;
mod crypto;
mod csv;
mod http;
mod opts;
mod passwd;

pub use base64::*;
pub use crypto::*;
pub use csv::*;
pub use http::*;
pub use opts::*;
pub use passwd::*;

pub fn parse_base64_format(format: &str) -> Result<Base64Format, &'static str> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "url" => Ok(Base64Format::UrlSafe),
        _ => Err("Invalid base64 format."),
    }
}

#[cfg(test)]
mod tests {
    use crate::verify_input_file;

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
