pub mod registe_center;

mod account;
mod database;
mod device;
mod storage;
mod wasmer;

use idns_eth_api::idns::system::{Command, CommandResponse};
pub use idns_eth_api::{EmptyMessage, Error, Handler, Result};
use std::collections::HashMap;

use account::{AccountServiceImpl, AuthServiceImpl};
use database::DatabaseServiceImpl;
use device::DeviceServiceImpl;
use storage::StorageServiceImpl;
use wasmer::SdkTaskServiceImpl;

use tokio::runtime::Handle;
//
pub fn execute(request: Command) -> Result<CommandResponse> {
    let handle = Handle::current();
    let handle_std =
        std::thread::spawn(move || handle.block_on(async move { async_execute(request).await }));

    if let Ok(res) = handle_std.join() {
        res
    } else {
        //
        Ok(CommandResponse {
            headers: HashMap::<String, String>::new(),
            status: 3,
            data: vec![],
            nonce: String::new(),
        })
    }
}

//
pub async fn async_execute(request: Command) -> Result<CommandResponse> {
    //通过不同的命令进行执行
    let service_name = request.service_name.clone();

    if service_name.starts_with("idns.system") {
        if service_name.starts_with("idns.system.account") {
            return AccountServiceImpl::new().execute(request).await;
        } else if service_name.starts_with("idns.system.auth") {
            return AuthServiceImpl::new().execute(request).await;
        } else if service_name.starts_with("idns.system.storage") {
            return StorageServiceImpl::new().execute(request).await;
        } else if service_name.starts_with("idns.system.device") {
            return DeviceServiceImpl::new().execute(request).await;
        } else if service_name.starts_with("idns.system.database") {
            return DatabaseServiceImpl::new().execute(request).await;
        } else if service_name.starts_with("idns.system.wasmer.task") {
            return SdkTaskServiceImpl::new().execute(request).await;
        }
    }

    //
    Ok(CommandResponse {
        headers: HashMap::<String, String>::new(),
        status: 3,
        data: vec![],
        nonce: String::new(),
    })
}
