mod client;
mod server;
pub use client::*;
pub use server::*;

use anyhow::Result;
use tonic::{metadata::MetadataValue, transport::Server, Request, Response, Status, Streaming};

fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    // let token = MetadataValue::from_str("Bearer some-secret-token").unwrap();

    // req.metadata_mut().insert("authorization", token.clone());

    // match req.metadata().get("authorization") {
    //     Some(t) if token == t => Ok(req),
    //     _ => Err(Status::unauthenticated("No valid auth token")),
    // }
    Ok(req)
}

pub async fn server_start() -> Result<()> {
    tracing::info!("GrpcServer Start.....");
    //启动server
    Server::builder()
        .accept_http1(true)
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(crate::grpc::FILE_DESCRIPTOR_SET)
                .build()
                .unwrap(),
        )
        .add_service(tonic_web::enable(RpcServiceServer::with_interceptor(
            RpcServiceImpl::new(),
            check_auth,
        )))
        .serve("0.0.0.0:39080".parse().unwrap())
        .await?;
    tracing::info!("GrpcServer Stop.....");
    //
    Ok(())
}
