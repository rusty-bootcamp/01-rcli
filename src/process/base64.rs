use anyhow::Ok;
use base64::{Engine as _, prelude::*};

use crate::{Base64Format, input_reader};

pub fn process_encode(input: &str, format: &Base64Format) -> anyhow::Result<String> {
    let content = input_reader(input)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&content),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: &Base64Format) -> anyhow::Result<String> {
    let content = input_reader(input)?;
    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(&content),
    }?;

    let decoded = String::from_utf8(decoded)?;

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_codec() -> anyhow::Result<()> {
        let encoded = process_encode("Hello, World!", &Base64Format::Standard)?;
        let decoded = process_decode(&encoded, &Base64Format::Standard)?;
        assert_eq!("Hello, World!", decoded);
        Ok(())
    }
}
