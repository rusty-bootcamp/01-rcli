use rand::seq::{IteratorRandom, SliceRandom};

use crate::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_passwd(opts: &GenPassOpts) -> anyhow::Result<String> {
    let mut rng = rand::rng();

    let required_char_types = [opts.uppercase, opts.lowercase, opts.number, opts.symbol]
        .iter()
        .filter(|&&enabled| enabled)
        .count();

    if opts.length < required_char_types {
        return Err(anyhow::anyhow!(
            "Password length must be at least {}",
            required_char_types
        ));
    }

    let mut all_chars = Vec::new();
    let mut char_sets = Vec::new();

    if opts.uppercase {
        all_chars.extend_from_slice(UPPER);
        char_sets.push(UPPER);
    }

    if opts.lowercase {
        all_chars.extend_from_slice(LOWER);
        char_sets.push(LOWER);
    }

    if opts.number {
        all_chars.extend_from_slice(NUMBER);
        char_sets.push(NUMBER);
    }

    if opts.symbol {
        all_chars.extend_from_slice(SYMBOL);
        char_sets.push(SYMBOL);
    }

    if all_chars.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one character type must be enabled"
        ));
    }

    let mut passwd = Vec::with_capacity(opts.length);

    for chat_set in &char_sets {
        let c = chat_set
            .iter()
            .choose(&mut rng)
            .expect("Character set should not be empty");

        passwd.push(*c);
    }

    // Fill the rest of the password with random characters from all_chars
    for _ in passwd.len()..opts.length {
        let c = all_chars
            .iter()
            .choose(&mut rng)
            .expect("all_chars should not be empty");

        passwd.push(*c);
    }

    // Shuffle the password to randomize the positions of the characters
    passwd.shuffle(&mut rng);

    // Convert to string
    let passwd_string = String::from_utf8(passwd)?;

    Ok(passwd_string)
}
