use crate::chain_api::identity::ExternalApiAttribute;
use anyhow::{anyhow, Context, Result};

use idns_eth_api::idns::identity::{
    AttributeCreateRequest, AttributeEntity, AttributeUpdateRequest,
};
pub struct IdnsAttribute {}

impl IdnsAttribute {
    /// 切换身份
    pub async fn query_one_attribute(id: &String) -> Result<AttributeEntity> {
        ExternalApiAttribute::query_one_attribute(id).await
    }
    /// 获取身份列表
    pub async fn attribute_list(source_identity: &String) -> Result<Vec<AttributeEntity>> {
        ExternalApiAttribute::attribute_list(source_identity).await
    }
    /// 获取身份列表
    pub async fn create_attribute(create_request: &AttributeCreateRequest) -> Result<()> {
        ExternalApiAttribute::create_attribute(create_request).await
    }
    /// 获取身份列表
    pub async fn remove_attribute(id: &String) -> Result<()> {
        ExternalApiAttribute::remove_attribute(id).await
    }
    /// 获取身份列表
    pub async fn update_attribute(request: &AttributeUpdateRequest) -> Result<()> {
        ExternalApiAttribute::update_attribute(request).await
    }
}
