use anyhow::{anyhow, Result};

pub fn get_local_ip() -> Result<String> {
    if cfg!(target_os = "linux") || cfg!(target_os = "windows") || cfg!(target_os = "macos") {
        use local_ip_address::local_ip;

        let my_local_ip = local_ip()?;

        Ok(format!("{}", my_local_ip))
    } else {
        Err(anyhow!("不支持的系统"))
    }
}
