pub use crate::idns::system::{Command, CommandResponse, EmptyMessage};
use crate::Error;
use anyhow::Result as AnyhowResult;
use prost::Message;
use std::collections::HashMap;

pub type Result<T> = AnyhowResult<T, Error>;

#[async_trait::async_trait]
pub trait Handler {
    //
    async fn execute(&self, command: Command) -> Result<CommandResponse>;
}

//
pub struct EmptyHandler;

#[async_trait::async_trait]
impl Handler for EmptyHandler {
    async fn execute(&self, _command: Command) -> Result<CommandResponse> {
        Ok(CommandResponse {
            headers: HashMap::<String, String>::new(),
            status: 0,
            data: vec![],
            nonce: String::new(),
        })
    }
}

pub fn response<T>(message: Result<T>) -> Result<CommandResponse>
where
    T: Message,
{
    if let Ok(response) = message
        .map(|res| CommandResponse {
            headers: HashMap::<String, String>::new(),
            status: 0,
            data: res.encode_to_vec(),
            nonce: String::new(),
        })
        .map_err(|err| {
            tracing::error!("调用service返回错误:{:#}", err);
            match err {
                Error::NormalError(code, message) => {
                    tracing::error!("业务错误码:{:#},错误消息:{:#}", code, message);
                    error_command_response(code, message.clone())
                }
                Error::SendExtrinsicError => {
                    error_command_response(4, String::from("调用合约失败!"))
                }
                Error::SubstrateConnectFailed => {
                    error_command_response(8, String::from("Substrate连接失败!"))
                }
                Error::IpfsConnectFailed => {
                    error_command_response(16, String::from("IPFS连接失败!"))
                }
                Error::NotFoundService => {
                    error_command_response(32, String::from("没有查找到服务!"))
                }
                _ => error_command_response(1, String::from("系统错误!")),
            }
        })
    {
        Ok(response)
    } else {
        Err(Error::NormalError(1, String::from("系统错误!")))
    }
}

fn error_command_response(status: i32, message: String) -> CommandResponse {
    let mut headers = HashMap::<String, String>::new();
    headers.insert(String::from("error_message"), message);

    CommandResponse {
        headers,
        status,
        data: vec![],
        nonce: String::new(),
    }
}
