pub mod registe_center;

mod account;
mod database;
pub(crate) mod device;
mod identity;
mod storage;
mod wasmer;

use idns_eth_api::idns::system::{Command, CommandResponse};
pub use idns_eth_api::{EmptyMessage, Error, Handler, Result};
use std::collections::HashMap;

use account::AccountServiceImpl;
use database::DatabaseServiceImpl;
use device::DeviceServiceImpl;
use identity::{
    AttributeServiceImpl, AttributeValueServiceImpl, IdentityServiceImpl, MetaCredentialServiceImpl,
};
use storage::StorageServiceImpl;
use wasmer::SdkTaskServiceImpl;

pub(crate) use account::AuthServiceImpl;

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
        } else if service_name.starts_with("idns.system.identity") {
            if service_name.starts_with("idns.system.identity.identity") {
                return IdentityServiceImpl::new().execute(request).await;
            } else if service_name.starts_with("idns.system.identity.meta_credential") {
                return MetaCredentialServiceImpl::new().execute(request).await;
            } else if service_name.starts_with("idns.system.identity.attribute.") {
                return AttributeServiceImpl::new().execute(request).await;
            } else if service_name.starts_with("idns.system.identity.attribute_value") {
                return AttributeValueServiceImpl::new().execute(request).await;
            }
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
    Err(Error::NotFoundService)
}
