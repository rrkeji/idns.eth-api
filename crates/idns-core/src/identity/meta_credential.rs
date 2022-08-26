use crate::chain_api::identity::ExternalApiMetaCredential;
use anyhow::{anyhow, Context, Result};

use idns_eth_api::idns::identity::{
    MetaCredentialCreateRequest, MetaCredentialEntity, MetaCredentialUpdateRequest,
};
pub struct IdnsMetaCredential {}

impl IdnsMetaCredential {
    /// 切换身份
    pub async fn query_one_meta_credential(
        source_identity: &String,
        name: &String,
    ) -> Result<MetaCredentialEntity> {
        ExternalApiMetaCredential::query_one_meta_credential(source_identity, name).await
    }
    /// 获取身份列表
    pub async fn meta_credential_list(
        source_identity: &String,
    ) -> Result<Vec<MetaCredentialEntity>> {
        ExternalApiMetaCredential::meta_credential_list(source_identity).await
    }
    /// 获取身份列表
    pub async fn create_meta_credential(
        create_request: &MetaCredentialCreateRequest,
    ) -> Result<()> {
        ExternalApiMetaCredential::create_meta_credential(create_request).await
    }
    /// 获取身份列表
    pub async fn remove_meta_credential(source_identity: &String, name: &String) -> Result<()> {
        ExternalApiMetaCredential::remove_meta_credential(source_identity, name).await
    }
    /// 获取身份列表
    pub async fn update_meta_credential(request: &MetaCredentialUpdateRequest) -> Result<()> {
        ExternalApiMetaCredential::update_meta_credential(request).await
    }
}
