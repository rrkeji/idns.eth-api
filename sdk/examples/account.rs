use bytes::Bytes;
use idns_eth_api::idns::account::{
    Account, EncryptMessageRequest, LoginRequest, LoginResponse as R,
};
use prost::Message;
use std::collections::HashMap;
use std::{thread, time};

use idns_eth_api::{response, Command, CommandResponse, EmptyMessage, Error, Handler, Result};
use idns_eth_core::account::{ALICE_PHRASE, ALICE_PUBLIC_KEY};
// [0, "chest shaft youth voice large vast idle mosquito train vacant license pole", "0x2e15c089cd236ac8f63efa16d910ca7bbf045e6d01d45431159745bd23977c15"]
use idns_eth_sdk::execute;

use idns_eth_api::idns::storage::{FileEntity, ListFilesRequest, ListFilesResponse};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("测试.");

    idns_eth_sdk::init_sdk(ALICE_PUBLIC_KEY).unwrap();

    let ten_millis = time::Duration::from_millis(1000000);
    thread::sleep(ten_millis);

    // let data = LoginRequest {
    //     password: String::from("123"),
    //     phrase: String::from(ALICE_PHRASE),
    // };

    // let command_request = Command {
    //     service_name: String::from("idns.system.auth"),
    //     method_name: String::from("import_and_login"),
    //     headers: HashMap::<String, String>::new(),
    //     data: data.encode_to_vec(),
    //     nonce: String::new(),
    // };
    // //
    // if let Ok(response) = execute(command_request) {
    //     //
    //     let headers = response.headers;
    //     let mut message = String::from("");

    //     let error_key = String::from("error_message");
    //     if headers.contains_key(&error_key) {
    //         message = headers.get(&error_key).unwrap().clone();
    //     }
    //     if response.status == 0 {
    //         if let Ok(response_data) = R::decode(Bytes::from(response.data)) {
    //             println!(
    //                 "{}--{}--{}",
    //                 response_data.public_key, response_data.application_key, response_data.token
    //             );
    //             //获取文件列表
    //             list_files();
    //         } else {
    //             println!("序列化错误!");
    //         }
    //     } else {
    //         println!("返回的不是0");
    //     }
    // } else {
    //     println!("系统错误!");
    // }
}

fn list_files() {
    let data = ListFilesRequest { parent_id: 0 };

    let command_request = Command {
        service_name: String::from("idns.system.storage"),
        method_name: String::from("list_files"),
        headers: HashMap::<String, String>::new(),
        data: data.encode_to_vec(),
        nonce: String::new(),
    };
    //
    if let Ok(response) = execute(command_request) {
        //
        let headers = response.headers;
        let mut message = String::from("");

        let error_key = String::from("error_message");
        if headers.contains_key(&error_key) {
            message = headers.get(&error_key).unwrap().clone();
        }
        if response.status == 0 {
            if let Ok(response_data) = ListFilesResponse::decode(Bytes::from(response.data)) {
                println!("---{}===", response_data.files.len());
                //获取文件列表
            } else {
                println!("序列化错误!");
            }
        } else {
            println!("返回的不是0");
        }
    } else {
        println!("系统错误!");
    }
}
