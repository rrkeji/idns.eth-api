use idns_eth_core::{account::Account, kvstore::KVStore};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("程序启动读取配置文件.");

    let (public_key, phrase) = Account::generate_account().unwrap();

    tracing::info!(public_key);
    tracing::info!(phrase);
    //
    let _ = KVStore::set_value("xxxxxx", "").await.unwrap();

    let value = KVStore::get_value("test").await.unwrap();
    tracing::info!(value);
}
