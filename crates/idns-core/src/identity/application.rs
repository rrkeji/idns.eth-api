use crate::chain_api::identity::ExternalApiApplication;
use anyhow::{anyhow, Context, Result};

use idns_eth_api::idns::identity::{
    ApplicationCreateRequest, ApplicationEntity, ApplicationUpdateRequest,
};
pub struct IdnsApplication {}

impl IdnsApplication {
    /// 切换身份
    pub async fn query_one_application(id: &String) -> Result<ApplicationEntity> {
        ExternalApiApplication::query_one_application(id).await
    }
    /// 获取身份列表
    pub async fn application_mylist(provider: &String) -> Result<Vec<ApplicationEntity>> {
        ExternalApiApplication::application_mylist(provider).await
    }
    /// 获取身份列表
    pub async fn create_application(create_request: &ApplicationCreateRequest) -> Result<()> {
        ExternalApiApplication::create_application(create_request).await
    }
    /// 获取身份列表
    pub async fn remove_application(id: &String) -> Result<()> {
        ExternalApiApplication::remove_application(id).await
    }
    /// 获取身份列表
    pub async fn update_application(request: &ApplicationUpdateRequest) -> Result<()> {
        ExternalApiApplication::update_application(request).await
    }
}
