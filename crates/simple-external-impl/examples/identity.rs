use anyhow::Result;

use simple_external_impl::identity::ExternalApiIdentity;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");
    ExternalApiIdentity::identities(&String::from(
        "b68a805c4064be1d49529abd0147fc2f08962c0594f0827d8b48b8864abd0625",
    ))
    .await;
    Ok(())
}
