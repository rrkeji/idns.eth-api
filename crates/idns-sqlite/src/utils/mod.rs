mod ipfs_client_utils;
mod rusqlite_utils;

pub use ipfs_client_utils::*;
pub use rusqlite_utils::*;

use anyhow::{anyhow, Result};

/// 通过应用的app 可以获取默认的数据库的路径
pub fn get_database_path(applicatin_key: &String) -> Result<String> {
    use idns_eth_core::idns_home_path;
    //
    let storage_path = idns_home_path()?.join(applicatin_key);
    let _ = std::fs::create_dir_all(storage_path.as_path());

    storage_path
        .join("sqlite.db")
        .as_path()
        .to_str()
        .map_or(Err(anyhow!("")), |r| Ok(String::from(r)))
}
