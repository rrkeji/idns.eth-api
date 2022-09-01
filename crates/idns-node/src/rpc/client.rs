use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use idns_eth_api::idns::system::{Command, CommandResponse};
use prost::Message;

pub use crate::grpc::rpc_service_client::RpcServiceClient;
pub use crate::grpc::rpc_service_server::{RpcService, RpcServiceServer};

use crate::grpc::{RpcCommand, RpcCommandResponse};

pub async fn request(host: &String, command: Command) -> Result<CommandResponse> {
    // TODO pool
    // let mut client = RpcServiceClient::connect(host.clone()).await;
    Err(anyhow!("没有实现")).context("")?
}

pub async fn async_request(host: &String, command: Command) -> Result<CommandResponse> {
    tracing::debug!("async_request hoost:{}", host);
    // TODO pool
    let mut client = RpcServiceClient::connect(host.clone())
        .await
        .context("建立GRPC链接")?;

    tracing::debug!("建立GRPC链接完成!");

    //command to rpcCommand
    let response = client
        .request(
            RpcCommand::decode(Bytes::from(command.encode_to_vec())).context("command格式转换")?,
        )
        .await
        .context("调用GRPC")?;

    tracing::debug!("调用GRPC完成:{:?}", response);
    Ok(
        CommandResponse::decode(Bytes::from(response.into_inner().encode_to_vec()))
            .context("返回值格式转换")?,
    )
}

pub fn registe() -> Result<()> {
    Ok(())
}
