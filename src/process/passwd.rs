use rand::seq::{IteratorRandom, SliceRandom};

const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_passwd(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<Vec<u8>> {
    let mut rng = rand::thread_rng();

    let required_char_types = [uppercase, lowercase, number, symbol]
        .iter()
        .filter(|&&enabled| enabled)
        .count();

    if length < required_char_types {
        return Err(anyhow::anyhow!(
            "Password length must be at least {}",
            required_char_types
        ));
    }

    let mut all_chars = Vec::new();
    let mut char_sets = Vec::new();

    if uppercase {
        all_chars.extend_from_slice(UPPER);
        char_sets.push(UPPER);
    }

    if lowercase {
        all_chars.extend_from_slice(LOWER);
        char_sets.push(LOWER);
    }

    if number {
        all_chars.extend_from_slice(NUMBER);
        char_sets.push(NUMBER);
    }

    if symbol {
        all_chars.extend_from_slice(SYMBOL);
        char_sets.push(SYMBOL);
    }

    if all_chars.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one character type must be enabled"
        ));
    }

    let mut passwd = Vec::with_capacity(length);

    // 确保每种启用的字符类型至少出现一次
    for char_set in &char_sets {
        let c = char_set
            .iter()
            .choose(&mut rng)
            .expect("Character set should not be empty");

        passwd.push(*c);
    }

    // 用随机字符填充密码的剩余部分
    for _ in passwd.len()..length {
        let c = all_chars
            .iter()
            .choose(&mut rng)
            .expect("all_chars should not be empty");

        passwd.push(*c);
    }

    // 随机排列密码中的字符
    passwd.shuffle(&mut rng);

    Ok(passwd)
}
