use anyhow::Ok;
use rcli::process_passwd;

#[test]
fn test_process_passwd_generating() -> anyhow::Result<()> {
    let passwd = process_passwd(16, false, true, true, true)?;
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
