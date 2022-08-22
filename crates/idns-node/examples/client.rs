use anyhow::Result;
use idns_eth_api::idns::account::{
    Account, EncryptMessageRequest, LoginRequest, LoginResponse as R,
};
use idns_eth_api::idns::system::{Command, CommandResponse, StringMessage};
use idns_eth_core::account::{login, ALICE_PHRASE, ALICE_PUBLIC_KEY};
use idns_eth_node::async_request;
use prost::Message;
use std::collections::HashMap;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("client测试.");

    // let token = login(
    //     &String::from("c85c0ddf8e2debd5883c1f53069e22fbea4b333c562ab3dee51b54485620a918"),
    //     ALICE_PUBLIC_KEY,
    //     ALICE_PHRASE,
    // )
    // .await?;
    // tracing::info!("Token{:?}", token);

    let data = LoginRequest {
        password: String::from("123"),
        phrase: String::from(ALICE_PHRASE),
    };
    let data = StringMessage {
        data: String::from("b68a805c4064be1d49529abd0147fc2f08962c0594f0827d8b48b8864abd0625"),
    };

    //request
    let command = Command {
        service_name: String::from("idns.system.identity.identity"),
        method_name: String::from("list_identities"),
        headers: HashMap::<String, String>::new(),
        data: data.encode_to_vec(),
        nonce: String::new(),
    };
    tracing::error!("command:{:?}.", command);
    let res: CommandResponse =
        async_request(&String::from("http://127.0.0.1:39080"), command).await?;
    tracing::error!("response:{:?}.", res);

    Ok(())
}
