use anyhow::Result;

use idns_eth_wasmer::executor::WasmExecutor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    tracing::debug!("download start........");

    let executor = WasmExecutor::new();

    let res = executor
        .execute(&String::from(
            "QmaF4dXrB8Kb3aebSLx9FQ5pd8djFgpRNTtMvNsWdb7Kvk",
        ))
        .await?;

    Ok(())
}
