use anyhow::Ok;
use base64::{Engine as _, prelude::*};

use crate::{Base64Format, DecodeOpts, EncodeOpts, input_reader};

pub fn process_encode(opts: &EncodeOpts) -> anyhow::Result<String> {
    let content = input_reader(&opts.input)?;
    let encoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.encode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&content),
    };

    Ok(encoded)
}

pub fn process_decode(opts: &DecodeOpts) -> anyhow::Result<String> {
    let content = input_reader(&opts.input)?;
    let decoded = match opts.format {
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
        let opts = EncodeOpts {
            input: "Hello, World!".to_string(),
            format: Base64Format::Standard,
        };
        let encoded = process_encode(&opts)?;
        let decoded = process_decode(&DecodeOpts {
            input: encoded.clone(),
            format: Base64Format::Standard,
        })?;
        assert_eq!("Hello, World!", decoded);

        Ok(())
    }
}
