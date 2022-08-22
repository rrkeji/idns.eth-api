pub(crate) mod command;
pub(crate) mod utils;

pub use command::*;
pub use utils::*;

pub use idns_eth_node::{get_connection, get_token};
pub use idns_eth_sqlite as sqlite;

use anyhow::Result;
pub use idns_eth_core::account::IdnsToken;

/// 初始化应用,程序启动
pub fn init_application(application_key: &str) -> Result<()> {
    idns_eth_node::init_application(application_key)
}

/// 初始化账户,登录之后
pub fn init_node(token: &IdnsToken) -> Result<()> {
    idns_eth_node::init_node(token)?;
    Ok(())
}
