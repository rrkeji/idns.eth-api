use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct DeviceInfo {
    pub device_type: Option<String>,
    pub os_type: Option<String>,
    pub mac: Option<String>,
    pub vpnc_address: Option<String>,
}

pub fn get_device_info() -> Result<DeviceInfo> {
    //自身设备局域网接入,并返回相关的连接信息
    let _ = crate::vpnc::launch()?;
    let vpnc_address = Some(String::from("10.0.0.2"));
    let device_type = Some(String::from("PC"));

    //
    if cfg!(target_os = "linux") || cfg!(target_os = "windows") || cfg!(target_os = "macos") {
        // use sys_info::{DiskInfo, MemInfo};

        //os_type
        let os_type = match sys_info::os_type() {
            Ok(os_type_str) => Some(os_type_str),
            _ => None,
        };
        //mac address
        let mac = match mac_address::get_mac_address() {
            Ok(Some(mac_res)) => Some(mac_res.to_string()),
            _ => None,
        };

        return Ok(DeviceInfo {
            os_type,
            mac,
            vpnc_address,
            device_type,
        });
    } else {
        Err(anyhow!("不支持的系统"))
    }
}