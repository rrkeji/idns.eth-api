pub(crate) mod command;
pub(crate) mod utils;

pub use command::*;
pub use utils::*;

pub use idns_eth_core as idns_core;
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

/// 登录
pub async fn import_and_login(phrase: &String, password: &String) -> Result<IdnsToken> {
    idns_eth_node::import_and_login(phrase, password).await
}

/// 判断是否已经导入账号
pub async fn is_imported() -> Result<bool> {
    let res = idns_eth_node::is_imported().await?;

    Ok(res)
}

/// 通过密码登录
pub async fn login_by_password(password: &String) -> Result<IdnsToken> {
    let res = idns_eth_node::login_by_password(password).await?;

    Ok(res)
}
