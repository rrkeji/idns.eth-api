#[macro_use]
extern crate lazy_static;

pub use idns_eth_core as idns_core;
pub use idns_eth_networks as networks;
pub use idns_eth_sqlite as sqlite;

pub(crate) mod account;
pub(crate) mod command;
pub(crate) mod database;
pub(crate) mod device;
pub(crate) mod server;
pub(crate) mod storage;
pub(crate) mod utils;

pub use command::*;
pub use server::*;
pub use utils::*;

use anyhow::Result;

/// 初始化应用,程序启动
pub fn init_application(application_key: &str) -> Result<()> {
    //读取配置
    crate::set_application_key(application_key);
    //启动服务
    match server::server_main() {
        Ok(_) => {
            tracing::debug!("server启动成功!");
        }
        Err(err) => {
            tracing::error!("server启动失败:{}", err);
        }
    }
    Ok(())
}

/// 初始化账户,登录之后
pub fn init_account() -> Result<()> {
    //账户网络初始化， 返回当前的设备节点信息

    //插入并更新当前的设备节点信息
    match device::init_device_node() {
        Ok(_) => {
            tracing::debug!("初始化设备节点成功!");
        }
        Err(err) => {
            tracing::error!("初始化设备节点失败:{}", err);
        }
    }
    //启动运行本设备执行的服务

    Ok(())
}
