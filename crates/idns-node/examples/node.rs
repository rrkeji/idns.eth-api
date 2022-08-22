use anyhow::Result;
use idns_eth_core::account::{login, ALICE_PHRASE, ALICE_PUBLIC_KEY};
use idns_eth_node::{init_application, init_node, init_node_async};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");

    init_application("663613c15a4a7198b0da88d00ae40227d1901eb7da92bb20f3f4a9329a596529")?;

    let token = login(
        &String::from("663613c15a4a7198b0da88d00ae40227d1901eb7da92bb20f3f4a9329a596529"),
        ALICE_PUBLIC_KEY,
        ALICE_PHRASE,
    )
    .await?;
    tracing::info!("Token{:?}", token);

    init_node(&token);

    tracing::info!("Token。。。。。。。。。。。。。。");

    Ok(())
}
