use anyhow::Ok;
use rcli::{GenPassOpts, process_passwd};

#[test]
fn test_process_passwd_generating() -> anyhow::Result<()> {
    let opts = GenPassOpts {
        length: 16,
        uppercase: false,
        lowercase: true,
        number: true,
        symbol: true,
    };

    let passwd = process_passwd(&opts)?;
    let passwd = String::from_utf8(passwd)?;
    assert_eq!(passwd.len(), 16);

    let has_uppercase = passwd.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = passwd.chars().any(|c| c.is_ascii_lowercase());
    let has_number = passwd.chars().any(|c| c.is_ascii_digit());
    let has_symbol = passwd.chars().any(|c| "!@#$%^&*_".contains(c));

    assert!(!has_uppercase);
    assert!(has_lowercase);
    assert!(has_number);
    assert!(has_symbol);

    Ok(())
}
