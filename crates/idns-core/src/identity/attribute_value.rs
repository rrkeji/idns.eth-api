use crate::chain_api::identity::ExternalApiAttributeValue;
use anyhow::{anyhow, Context, Result};

use idns_eth_api::idns::identity::{AttributeValueCreateRequest, AttributeValueEntity};
pub struct IdnsAttributeValue {}

impl IdnsAttributeValue {
    /// 切换身份
    pub async fn query_one_attribute_value(id: &String) -> Result<AttributeValueEntity> {
        ExternalApiAttributeValue::query_one_attribute_value(id).await
    }
    /// 获取身份列表
    pub async fn attribute_value_list_by_holder(
        holder_identity: &String,
    ) -> Result<Vec<AttributeValueEntity>> {
        ExternalApiAttributeValue::attribute_value_list_by_holder(holder_identity).await
    }
    /// 获取身份列表
    pub async fn attribute_value_list_by_source(
        source_identity: &String,
    ) -> Result<Vec<AttributeValueEntity>> {
        ExternalApiAttributeValue::attribute_value_list_by_source(source_identity).await
    }
    /// 获取身份列表
    pub async fn create_attribute_value(
        create_request: &AttributeValueCreateRequest,
    ) -> Result<()> {
        ExternalApiAttributeValue::create_attribute_value(create_request).await
    }
    /// 获取身份列表
    pub async fn remove_attribute_value(id: &String) -> Result<()> {
        ExternalApiAttributeValue::remove_attribute_value(id).await
    }
}
