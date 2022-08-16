use anyhow::Result;

use idns_eth_networks::device::{get_device_info, DeviceInfo};

pub fn init_device_node() -> Result<()> {
    //获取设备的信息
    let device_info = get_device_info()?;
    //获取设备的

    Ok(())
}
