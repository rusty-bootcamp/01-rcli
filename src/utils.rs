use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

pub fn verify_input_file(input: &str) -> Result<String, &'static str> {
    let path = Path::new(input);
    if path.exists() && path.is_file() || input == "-" {
        Ok(input.into())
    } else {
        Err("File does not exist.")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory.")
    }
}

pub fn input_reader(input: &str) -> anyhow::Result<String> {
    let mut rdr: Box<dyn Read> = if input == "-" {
        // 从标准输入读取
        Box::new(std::io::stdin())
    } else if Path::new(input).exists() && Path::new(input).is_file() {
        // 从文件读取
        Box::new(File::open(input)?)
    } else {
        // 直接将输入字符串作为数据源
        Box::new(Cursor::new(input))
    };

    let mut buf = String::new();
    rdr.read_to_string(&mut buf)?;
    let buf = buf.trim();

    Ok(buf.to_string())
}
