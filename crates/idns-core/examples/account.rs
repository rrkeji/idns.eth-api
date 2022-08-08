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

    let token = login(&String::from("c85c0ddf8e2debd5883c1f53069e22fbea4b333c562ab3dee51b54485620a918"), ALICE_PUBLIC_KEY, ALICE_PHRASE).await?;
    tracing::info!("Token{:?}", token);
    let value = KVStore::get_value(&token).await?;
    tracing::info!("获取到的值{:?}", value);
    //
    let test = KVStore::set_value(&token, value.1, "xxxs2222sssssss222sssssx").await?;
    tracing::info!("setsss|||{:?}", test);
    let value = KVStore::get_value(&token).await?;
    tracing::info!("{:?}", value);
    Ok(())
}
