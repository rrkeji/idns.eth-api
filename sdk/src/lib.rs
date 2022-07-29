pub use idns_eth_core as idns_core;
pub use idns_eth_networks as networks;
pub use idns_eth_sqlite as sqlite;

pub(crate) mod account;

use account::AccountServiceImpl;
use bytes::Bytes;
pub use idns_eth_api::{Command, CommandResponse, EmptyMessage, Error, Handler, Result};
use prost::Message;
use std::collections::HashMap;

//
pub fn execute(request: Command) -> Result<CommandResponse> {
    //通过不同的命令进行执行
    let service_name = request.service_name.clone();

    if service_name.starts_with("idns.system") {
        if service_name.starts_with("idns.system.account") {
            return AccountServiceImpl::new().execute(request);
        }
    }

    Ok(CommandResponse {
        headers: HashMap::<String, String>::new(),
        status: 3,
        data: vec![],
        nonce: String::new(),
    })
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
