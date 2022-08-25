use anyhow::Result;

use idns_eth_core::account::{decrypt_message, encrypt_message, ALICE_PHRASE};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("程序启动读取配置文件.");
    let password = String::from("559271e2a29535c057e23ce3d7019b79");

    if let Ok((salt, account_id, nonce, cipher)) =
        encrypt_message(&String::from(ALICE_PHRASE), &password)
    {
        tracing::info!("salt:[{}]\nnonce:[{}]\ncipher:[{}]\n", salt, nonce, cipher);

        let plain = decrypt_message(&salt, &nonce, &cipher, &password)?;
        tracing::info!("plain:[{}]", plain);
    }
    //
    let salt = String::from("O5f5UGJCap0z_IHTDE6XWWw4e4_aGKWCN1wKicidwNU=");
    let cipher = String::from(
        "R0sIcj7oUYqC7z5bjHx-DghxbBMmgPAo0mmTZ66k7sutLnW1vDnL2a9iVvS71S4SNjD427ZsKvajsyqW",
    );
    let nonce = String::from("THw4UiuIvquhPrKqHPg5YRSb5Q==");

    let plain = decrypt_message(&salt, &nonce, &cipher, &password)?;
    tracing::info!("plain:[{}]", plain);
    Ok(())
}
