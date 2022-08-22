pub use crate::grpc::rpc_service_server::{RpcService, RpcServiceServer};
use crate::grpc::{RpcCommand, RpcCommandResponse};
use bytes::Bytes;
use idns_eth_api::idns::system::Command;
use prost::Message;
use tonic::{Request, Response, Status};

pub struct RpcServiceImpl;

impl RpcServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl RpcService for RpcServiceImpl {
    //
    async fn request(
        &self,
        request: Request<RpcCommand>,
    ) -> Result<Response<RpcCommandResponse>, Status> {
        tracing::info!("Got a request: {:?}", request);
        let message = request.into_inner();
        let service_name = message.service_name.clone();

        if service_name.starts_with("idns.system.") {
            //系统服务
            let response = crate::services::async_execute(
                Command::decode(Bytes::from(message.encode_to_vec()))
                    .map_err(|err| Status::unknown(format!("{}", err)))?,
            )
            .await
            .map_err(|err| Status::unknown(format!("{}", err)))?;

            return Ok(Response::new(
                RpcCommandResponse::decode(Bytes::from(response.encode_to_vec()))
                    .map_err(|err| Status::unknown(format!("{}", err)))?,
            ));
        }
        //从注册中心查找
        if let Some(arc_handler) = crate::get_service_handler(&service_name) {
            let response = arc_handler(
                Command::decode(Bytes::from(message.encode_to_vec()))
                    .map_err(|err| Status::unknown(format!("{}", err)))?,
            )
            .map_err(|err| Status::unknown(format!("{}", err)))?;

            return Ok(Response::new(
                RpcCommandResponse::decode(Bytes::from(response.encode_to_vec()))
                    .map_err(|err| Status::unknown(format!("{}", err)))?,
            ));
        }
        //TODO 判断是否需要中转
        Err(Status::unimplemented("没有实现或者没有注册"))
    }
}
