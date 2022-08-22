mod registe_center;

pub use registe_center::*;

use anyhow::Result;
use idns_eth_api::idns::system::{Command, CommandResponse};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    pub(crate) static ref SERVICES: RwLock<HashMap<String, Arc<dyn Fn(Command) -> Result<CommandResponse> + Send + Sync + 'static>>> =
        RwLock::new(HashMap::new());
}

/// 注册服务
pub fn registe_service(
    service_name: &String,
    handler: Arc<dyn Fn(Command) -> Result<CommandResponse> + Send + Sync + 'static>,
) -> Result<()> {
    let mut cache = SERVICES.write().unwrap();
    cache.insert(service_name.clone(), handler.clone());
    Ok(())
}
/// 服务是否存在
pub fn service_exist(service_name: &String) -> bool {
    SERVICES.read().unwrap().contains_key(service_name)
}
/// 获取服务
pub fn get_service_handler(
    service_name: &String,
) -> Option<Arc<dyn Fn(Command) -> Result<CommandResponse> + Send + Sync + 'static>> {
    if let Some(arc_handler) = SERVICES.read().unwrap().get(service_name) {
        return Some(arc_handler.clone());
    } else {
        None
    }
}
//取消注册服务
