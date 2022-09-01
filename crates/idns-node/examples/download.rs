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
    idns_eth_node::http::Server::download(&UdiApplication {
        identity: String::from("did:idns:CzsKrSUkffKnzPMZJWhNAgfgGAdYvdX7YHzYNZZJYSws"),
        provider: String::from("did:idns:CzsKrSUkffKnzPMZJWhNAgfgGAdYvdX7YHzYNZZJYSws"),
        application_cid: String::from("QmTGDGnCqcJCg9gK9Lzy1ZXWvMjKB9AYrnuCyQ6U6fZu4e"),
        application_type: String::from("Web"),
        name: String::from("Web应用测试"),
        labels: vec![String::from("web")],
        signature: String::from("2pgG4NsA62nH7TVz5YYZZGJX7zvA9P4vsfpm2jduWKTUYhHdZhC2o9woGh2siXRB5vFiJ2X62mFTrG1bPjYmsj8N"),
    })
    .await?;
    tracing::info!("。。。。。。。。。。。。。。");
    Ok(())
}
