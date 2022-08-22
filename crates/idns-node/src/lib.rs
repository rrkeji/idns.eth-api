#[macro_use]
extern crate lazy_static;

pub(crate) mod node;
pub(crate) mod rpc;
pub(crate) mod services;
pub(crate) mod utils;

//public
pub use rpc::{async_request, request};
pub use services::registe_center::{get_service_handler, registe_service};
pub use services::{async_execute as service_async_execute, execute as service_execute};
pub use utils::{get_connection, get_token};
//
pub(crate) use idns_eth_core as idns_core;
pub(crate) use idns_eth_networks as networks;
pub(crate) use idns_eth_sqlite as sqlite;
pub(crate) use utils::*;
//
pub mod grpc {
    pub const FILE_DESCRIPTOR_SET: &'static [u8] =
        tonic::include_file_descriptor_set!("idns_node_descriptor");

    //
    tonic::include_proto!("idns.node.rpc");
}

use anyhow::{anyhow, Context, Result};
use idns_eth_core::account::IdnsToken;
use idns_eth_sqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    pub(crate) static ref SYS_CONNECTION: RwLock<Option<Arc<Connection>>> = RwLock::new(None);
    pub(crate) static ref TOKEN: RwLock<Option<IdnsToken>> = RwLock::new(None);
    pub(crate) static ref PASSWORD: RwLock<Option<String>> = RwLock::new(None);
    //application key secret
    pub static ref APPLICATION_KEY: RwLock<Option<String>> = RwLock::new(None);
    pub static ref APPLICATION_SECRET: RwLock<Option<String>> = RwLock::new(None);
}

/// 初始化应用,程序启动
pub fn init_application(application_key: &str) -> Result<()> {
    //读取配置
    crate::set_application_key(application_key);
    Ok(())
}

use tokio::runtime::Handle;

pub fn init_node(token: &IdnsToken) -> Result<()> {
    //设置token
    tracing::debug!("设置token:{:?}", token);
    crate::utils::set_token(token)?;
    //TODO 检测IPFS服务等是否开启

    //启动系统数据库
    tracing::debug!("启动系统数据库");
    let arc_conn = Arc::new(Connection::open(token)?);
    crate::utils::set_connection(arc_conn.clone()).context("启动系统数据库")?;

    //启动网络服务
    tracing::debug!("启动网络服务");
    idns_eth_networks::launch().context("启动网络服务")?;

    //启动注册wasmer服务

    //启动设备服务

    let handle = Handle::current();
    let _ = std::thread::spawn(move || {
        handle.block_on(async move {
            //启动核心应用服务
            tracing::debug!("启动核心应用服务");
            // crate::rpc::server_start().await;
        })
    });

    Ok(())
}

pub async fn init_node_async(token: &IdnsToken) -> Result<()> {
    //设置token
    tracing::debug!("设置token");
    crate::utils::set_token(token)?;
    //TODO 检测IPFS服务等是否开启

    //启动系统数据库
    tracing::debug!("启动系统数据库");
    let arc_conn = Arc::new(Connection::open(token)?);
    crate::utils::set_connection(arc_conn.clone()).context("启动系统数据库")?;

    //启动网络服务
    tracing::debug!("启动网络服务");
    idns_eth_networks::launch().context("启动网络服务")?;

    //启动注册wasmer服务

    //启动设备服务

    //启动核心应用服务
    tracing::debug!("启动核心应用服务");
    crate::rpc::server_start()
        .await
        .context("启动核心应用服务")?;

    Ok(())
}
