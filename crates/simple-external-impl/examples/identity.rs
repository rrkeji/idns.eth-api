use anyhow::Result;

use simple_external_impl::identity::ExternalApiIdentity;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");
    let res = ExternalApiIdentity {
        public_key: String::from(
            "7a0b9c92b65218204416d335c7b85ef9d47da1ba92bbb2b3a23224c6cd38ce54",
        ),
        signature: String::from(""),
        nonce: String::from(""),
    }
    .identities()
    .await;
    tracing::info!("返回:{:?}.", res);
    Ok(())
}
