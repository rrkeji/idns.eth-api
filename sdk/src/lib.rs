#[macro_use]
extern crate lazy_static;

pub use idns_eth_core as idns_core;
pub use idns_eth_networks as networks;
pub use idns_eth_sqlite as sqlite;

pub(crate) mod account;
pub(crate) mod database;
pub(crate) mod storage;
pub(crate) mod utils;

pub(crate) use utils::*;

use account::{AccountServiceImpl, AuthServiceImpl};
use storage::StorageServiceImpl;

use bytes::Bytes;
pub use idns_eth_api::{Command, CommandResponse, EmptyMessage, Error, Handler, Result};
use prost::Message;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use idns_eth_core::account::IdnsToken;
use idns_eth_sqlite::Connection;

lazy_static! {
    pub(crate) static ref SYS_CONNECTION: RwLock<Option<Arc<Connection>>> = RwLock::new(None);
    pub(crate) static ref TOKEN: RwLock<Option<IdnsToken>> = RwLock::new(None);
    pub(crate) static ref PASSWORD: RwLock<Option<String>> = RwLock::new(None);
    //application key secret
    pub static ref APPLICATION_KEY: RwLock<Option<String>> = RwLock::new(None);
    pub static ref APPLICATION_SECRET: RwLock<Option<String>> = RwLock::new(None);
}

//
pub fn execute(request: Command) -> Result<CommandResponse> {
    //通过不同的命令进行执行
    let service_name = request.service_name.clone();

    if service_name.starts_with("idns.system") {
        if service_name.starts_with("idns.system.account") {
            return AccountServiceImpl::new().execute(request);
        } else if service_name.starts_with("idns.system.auth") {
            return AuthServiceImpl::new().execute(request);
        } else if service_name.starts_with("idns.system.storage") {
            return StorageServiceImpl::new().execute(request);
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

pub fn set_application_key(applicatin_key: &str) {
    let mut w = crate::APPLICATION_KEY.write().unwrap();
    *w = Some(String::from(applicatin_key));
}

pub fn set_application_secret(applicatin_secret: &str) {
    let mut w = crate::APPLICATION_SECRET.write().unwrap();
    *w = Some(String::from(applicatin_secret));
}
