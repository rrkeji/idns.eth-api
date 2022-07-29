use bytes::Bytes;
use idns_eth_api::runnerc::account::{Account, EncryptMessageRequest, EncryptMessageResponse as R};
use prost::Message;
use std::collections::HashMap;

use idns_eth_api::{response, Command, CommandResponse, EmptyMessage, Error, Handler, Result};

// [0, "chest shaft youth voice large vast idle mosquito train vacant license pole", "0x2e15c089cd236ac8f63efa16d910ca7bbf045e6d01d45431159745bd23977c15"]
use idns_eth_sdk::execute;

fn main() {
    let data = EncryptMessageRequest {
        plain: String::from(
            "chest shaft youth voice large vast idle mosquito train vacant license pole",
        ),
        password: String::from("123456"),
    };

    let command_request = Command {
        service_name: String::from("runnerc.system.account"),
        method_name: String::from("encrypt_message"),
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
            if let Ok(response_data) = R::decode(Bytes::from(response.data)) {
                println!(
                    "{}--{}--{}",
                    response_data.cipher, response_data.nonce, response_data.salt
                );
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
