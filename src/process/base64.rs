use anyhow::Ok;
use base64::{Engine as _, prelude::*};

use crate::{Base64Format, DecodeOpts, EncodeOpts, file_reader};

pub fn process_encode(opts: &EncodeOpts) -> anyhow::Result<String> {
    let content = file_reader(&opts.input)?;
    let encoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.encode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&content),
    };
    println!("{:?}", encoded);
    Ok(encoded)
}

pub fn process_decode(opts: &DecodeOpts) -> anyhow::Result<String> {
    let content = file_reader(&opts.input)?;
    let decoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.decode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(&content),
    }?;

    let decoded = String::from_utf8(decoded)?;
    println!("{:?}", decoded);

    Ok(decoded)
}
