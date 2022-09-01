use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");
    let res = simple_external_impl::identity::identity_get_request(
        "/lcotwb_outtest/supplierInterface/shipment",
    )
    .await;
    tracing::info!("测试:{:?}", res);
    Ok(())
}
