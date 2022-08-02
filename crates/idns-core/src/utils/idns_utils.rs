use anyhow::{anyhow, Result};
use std::path::PathBuf;

/// 获取主目录
///
/// # 示例
///
/// ```
/// let storage_path = idns_home_path()?.join("sqlite");
/// std::fs::create_dir_all(storage_path.as_path()).unwrap();
/// ```
///
pub fn idns_home_path() -> Result<PathBuf> {
    let home_path = std::env::var_os("IDNS.ETH_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            home::home_dir().map(|tilde| {
                let mut path = PathBuf::from(tilde);
                path.push(".idns_eth");
                path
            })
        });

    //
    let home_path = Some(PathBuf::from("/Users/suhs/jinisu/idns.eth-api/target"));
    //
    if let Some(home_path) = home_path {
        //
        std::fs::create_dir_all(home_path.as_path())?;
        Ok(home_path)
    } else {
        Err(anyhow!("没有设置IDNS.ETH_PATH路径"))
    }
}
