use anyhow::Result;

use simple_external_impl::identity::{ExternalApiIdentity, ExternalApiVerifiableCredential};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");
    simple_external_impl::set_external_api_identity_signature((
        String::from(""),
        String::from(""),
        String::from(""),
    ))?;
    // let res = ExternalApiIdentity {}.identities().await;
    let res = ExternalApiVerifiableCredential::verifiable_credential_list_by_holder(&String::from(
        "did:idns:CzsKrSUkffKnzPMZJWhNAgfgGAdYvdX7YHzYNZZJYSws",
    ))
    .await;
    tracing::info!("返回:{:?}.", res);
    Ok(())
}
