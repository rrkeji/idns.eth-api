#[macro_use]
extern crate lazy_static;

pub use idns_eth_core as idns_core;
pub use idns_eth_networks as networks;
pub use idns_eth_sqlite as sqlite;

pub(crate) mod account;
pub(crate) mod command;
pub(crate) mod database;
pub(crate) mod server;
pub(crate) mod storage;
pub(crate) mod utils;

pub use command::*;
pub use server::*;
pub use utils::*;

use anyhow::Result;

pub fn init_sdk(application_key: &str) -> Result<()> {
    //读取配置
    crate::set_application_key(application_key);
    //启动服务
    server::server_main()?;
    Ok(())
}
