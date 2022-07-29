use anyhow::Result;
use idns_eth_core::{
    account::Account,
    account::{login, ALICE_PHRASE, ALICE_PUBLIC_KEY},
    kvstore::KVStore,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("程序启动读取配置文件.");

    // let (public_key, phrase) = Account::generate_account()?;

    // tracing::info!(public_key);
    // tracing::info!(phrase);

    let token = login(ALICE_PUBLIC_KEY, ALICE_PHRASE).await?;

    let value = KVStore::get_value(&token).await?;
    //
    let _ = KVStore::set_value(&token, value.1, "xxxs22222sssssx").await?;

    let value = KVStore::get_value(&token).await?;
    tracing::info!("{:?}", value);
    Ok(())
}
