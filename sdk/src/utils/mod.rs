use anyhow::{anyhow, Result};
use idns_eth_core::account::IdnsToken;
use idns_eth_sqlite::Connection;
use std::sync::Arc;

pub fn get_token() -> Result<IdnsToken> {
    if let Some(token) = crate::TOKEN.read().unwrap().as_ref() {
        return Ok(token.clone());
    } else {
        Err(anyhow!("没有Token!"))
    }
}

pub fn get_connection() -> Result<Arc<Connection>> {
    if let Some(arc_conn) = crate::SYS_CONNECTION.read().unwrap().as_ref() {
        return Ok(arc_conn.clone());
    } else {
        Err(anyhow!("没有系统数据库的链接!"))
    }
}

pub fn set_connection(conn: Arc<Connection>) -> Result<()> {
    let mut w = crate::SYS_CONNECTION.write().unwrap();
    *w = Some(conn);
    Ok(())
}

pub fn clear_connection() -> Result<()> {
    let mut w = crate::SYS_CONNECTION.write().unwrap();
    *w = None;
    Ok(())
}

pub fn get_Application_key() -> Result<String> {
    if let Some(str_value) = crate::APPLICATION_KEY.read().unwrap().as_ref() {
        return Ok(str_value.clone());
    } else {
        Err(anyhow!("没有设置Application Key!"))
    }
}

pub fn get_Application_secret() -> Result<String> {
    if let Some(str_value) = crate::APPLICATION_SECRET.read().unwrap().as_ref() {
        return Ok(str_value.clone());
    } else {
        Err(anyhow!("没有设置Application Secret!"))
    }
}

pub fn get_password() -> Result<String> {
    if let Some(str_value) = crate::PASSWORD.read().unwrap().as_ref() {
        return Ok(str_value.clone());
    } else {
        Err(anyhow!("没有输入Password!"))
    }
}
