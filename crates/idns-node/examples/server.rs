use anyhow::Result;
use idns_eth_api::idns::wasmer::UdiApplication;
use idns_eth_core::account::{login, ALICE_PHRASE, ALICE_PUBLIC_KEY};
use idns_eth_node::{init_application, init_node, init_node_async};
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");

    tracing::debug!("启动HTTP文件服务");
    idns_eth_node::http::Server::new().start()?;
    tracing::info!("。。。。。。。。。。。。。。");
    let ten_millis = time::Duration::from_millis(100000000000);
    thread::sleep(ten_millis);
    Ok(())
}
