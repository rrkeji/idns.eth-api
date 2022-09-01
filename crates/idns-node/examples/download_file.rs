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
    // idns_eth_node::http::Server::new().start()?;
    idns_eth_node::http::Server::download_file(
        &String::from("QmdoeABurHKENEvX4bMDp1TkMr7487p4SUzS84BUSpKvw6"),
        Some(String::from(".png")),
    )
    .await?;
    tracing::info!("。。。。。。。。。。。。。。");
    Ok(())
}
