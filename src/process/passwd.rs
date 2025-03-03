use rand::seq::IndexedRandom;

use crate::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_passwd(opts: &GenPassOpts) -> anyhow::Result<String> {
    let mut rng = rand::rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWER);
    }

    if opts.number {
        chars.extend_from_slice(NUMBER);
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..opts.length {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c as char);
    }

    Ok(password)
}
