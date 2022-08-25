use crate::services::device::DeviceServiceImpl;
use anyhow::{anyhow, Result};
use idns_eth_api::idns::networks::DeviceEntity;
use idns_eth_core::account::Account as AccountImpl;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct NodeInfo {
    pub device_type: Option<String>,
    pub os_type: Option<String>,
    pub mac: Option<String>,
    pub hostname: Option<String>,
    pub vpnc_address: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct DeviceJson {
    pub node_id: String,
    pub public_key: String,
    pub private_key: String,
}

pub fn init_device_node(ip: &String) -> Result<()> {
    //获取设备的ID
    let device_id = get_device_node_id()?;
    let device_service = DeviceServiceImpl::new();
    //查询该设备是否存在
    if let Some(device_entity) = device_service.find_by_uuid(&device_id)? {
        //获取设备的信息
        let device_info = get_node_info(&ip.clone())?;
        let device_info = &device_info;

        //存在,通过device_id插入更新
        device_service.update_device(&DeviceEntity {
            id: device_entity.id,
            owner_id: device_entity.owner_id,
            device_uuid: device_entity.device_uuid,
            name: device_entity.name,
            device_type: device_info
                .device_type
                .as_ref()
                .map_or(String::new(), |r| r.clone()),
            os_type: device_info
                .os_type
                .as_ref()
                .map_or(String::new(), |r| r.clone()),
            mac_address: device_info
                .mac
                .as_ref()
                .map_or(String::new(), |r| r.clone()),
            category: device_entity.category,
            vpnc_address: device_info
                .vpnc_address
                .as_ref()
                .map_or(String::new(), |r| r.clone()),
            icon_url: device_entity.icon_url,
            hostname: device_info
                .hostname
                .as_ref()
                .map_or(String::new(), |r| r.clone()),
            home: format!("{:?}", idns_eth_core::idns_home_path()?),
        })?;
    } else {
        let device_info = get_node_info(&ip.clone())?;
        let device_info = &device_info;

        let mac_address = device_info
            .mac
            .as_ref()
            .map_or(String::new(), |r| r.clone());

        //不存在,通过mac地址插入更新
        if let Some(device_entity) = device_service.find_by_mac_address(&mac_address)? {
            //存在,通过device_id插入更新
            device_service.update_device(&DeviceEntity {
                id: device_entity.id,
                owner_id: device_entity.owner_id,
                device_uuid: device_id,
                name: device_entity.name,
                device_type: device_info
                    .device_type
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                os_type: device_info
                    .os_type
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                mac_address: device_info
                    .mac
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                category: device_entity.category,
                vpnc_address: device_info
                    .vpnc_address
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                icon_url: device_entity.icon_url,
                hostname: device_info
                    .hostname
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                home: format!("{:?}", idns_eth_core::idns_home_path()?),
            })?;
        } else {
            let device_info = get_node_info(&ip.clone())?;
            let device_info = &device_info;
            //插入
            device_service.create_device(&DeviceEntity {
                id: 0,
                owner_id: String::new(),
                device_uuid: device_id,
                name: device_info
                    .os_type
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                device_type: device_info
                    .device_type
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                os_type: device_info
                    .os_type
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                mac_address: device_info
                    .mac
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                category: String::new(),
                vpnc_address: device_info
                    .vpnc_address
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                icon_url: String::new(),
                hostname: device_info
                    .hostname
                    .as_ref()
                    .map_or(String::new(), |r| r.clone()),
                home: format!("{:?}", idns_eth_core::idns_home_path()?),
            })?;
        }
    }

    Ok(())
}

pub fn get_tun_ip(device_id: &String) -> Result<String> {
    let device_service = DeviceServiceImpl::new();
    //查询该设备是否存在
    if let Some(device_entity) = device_service.find_by_uuid(device_id)? {
        Ok(device_entity.vpnc_address.clone())
    } else {
        let exists = device_service.list_device_vpnc_address()?;
        let ip_strs = exists.join(";");
        for itemp in 2..255 {
            let ip = format!("10.0.0.{:?}", itemp);
            if !ip_strs.contains(&ip) {
                return Ok(ip);
            }
        }
        Ok(String::from("10.0.1.9"))
    }
}

pub fn get_node_info(tun_ip: &String) -> Result<NodeInfo> {
    //自身设备局域网接入,并返回相关的连接信息
    // let _ = crate::vpnc::launch(server, tun_ip, tun_mask, key)?;
    let vpnc_address = Some(tun_ip.clone());
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
        //mac address
        let hostname = match sys_info::hostname() {
            Ok(hostname_str) => Some(hostname_str),
            _ => None,
        };

        return Ok(NodeInfo {
            os_type,
            mac,
            vpnc_address,
            device_type,
            hostname,
        });
    } else {
        Err(anyhow!("不支持的系统"))
    }
}

pub fn get_device_node_id() -> Result<String> {
    //获取设备， 首先查看本地的文件中是否有设备ID
    let exists = idns_eth_core::utils::files::file_exists("", "device.json")?;

    if exists {
        //存在
        let json_str = idns_eth_core::utils::files::read_string_from_file("", "device.json")?;

        let device: DeviceJson =
            serde_json::from_str(json_str.as_str()).map_err(|e| anyhow!("{}", e))?;

        return Ok(device.node_id.clone());
    } else {
        //
        let res = AccountImpl::generate_account()?;
        let file_content = json!(DeviceJson {
            node_id: res.0.clone(),
            public_key: res.0.clone(),
            private_key: res.1.clone(),
        })
        .to_string();

        if let Ok(_) = idns_eth_core::utils::files::write_to_file(
            "",
            "device.json",
            &file_content.as_bytes().to_vec(),
        ) {
            //保存密码
            return Ok(res.0.clone());
        }
    }
    Err(anyhow!("获取生成设备ID失败"))
}
