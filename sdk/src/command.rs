use bytes::Bytes;
pub use idns_eth_api::{Command, CommandResponse, EmptyMessage, Error, Handler, Result};
use prost::Message;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use idns_eth_core::account::IdnsToken;
use idns_eth_sqlite::Connection;

//
pub fn execute(request: Command) -> Result<CommandResponse> {
    //通过不同的命令进行执行
    idns_eth_node::service_execute(request)
}

/// request
pub fn idns_eth_request<T, R>(
    service_name: &str,
    method_name: &str,
    headers: HashMap<String, String>,
    data: T,
) -> (i32, String, Option<R>)
where
    T: Message,
    R: Message + std::default::Default,
{
    let command_request = Command {
        service_name: String::from(service_name),
        method_name: String::from(method_name),
        headers: headers,
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
                return (0, message, Some(response_data));
            } else {
                return (1, String::from("序列化错误!"), None);
            }
        } else {
            return (response.status, message, None);
        }
    } else {
        (1, String::from("系统错误!"), None)
    }
}

/// request
pub fn idns_eth_request_raw(
    service_name: &str,
    method_name: &str,
    headers: HashMap<String, String>,
    data: Vec<u8>,
) -> (i32, String, Vec<u8>) {
    let command_request = Command {
        service_name: String::from(service_name),
        method_name: String::from(method_name),
        headers: headers,
        data: data,
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
            return (0, message, response.data);
        } else {
            return (response.status, message, vec![]);
        }
    } else {
        (1, String::from("系统错误!"), vec![])
    }
}
