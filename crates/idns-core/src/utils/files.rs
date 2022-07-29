use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub fn write_to_file(path: &str, filename: &str, content: &Vec<u8>) -> Result<()> {
    //
    let storage_path = crate::utils::idns_home_path()?.join(path);
    std::fs::create_dir_all(storage_path.as_path())?;

    let filename = storage_path.join(filename);
    //创建文件
    let mut output = File::create(filename.as_path())?;
    output.write_all(content)?;
    Ok(())
}

pub fn read_string_from_file(path: &str, filename: &str) -> Result<String> {
    //
    let storage_path = crate::utils::idns_home_path()?.join(path);
    std::fs::create_dir_all(storage_path.as_path())?;

    let filename = storage_path.join(filename);
    let file = File::open(filename.as_path())?;
    let mut buf_reader = BufReader::new(file);
    // 解析配置文件
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}
