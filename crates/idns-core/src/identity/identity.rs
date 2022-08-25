use crate::chain_api::identity::ExternalApiIdentity;
use anyhow::{anyhow, Context, Result};
use chacha20poly1305::{
    aead::{stream, NewAead},
    XChaCha20Poly1305,
};
use idns_eth_api::idns::identity::{
    IdentityCreateRequest, IdentityEntity, IdentityUpdateRequest, ListIdentitiesResponse,
};
pub struct IdnsIdentity {}

impl IdnsIdentity {
    /// 本地设置当前的身份
    pub fn local_current_identity() -> Result<String> {
        Ok(String::from(""))
    }
    /// 切换身份
    pub fn change_identity(identity: &String) -> Result<()> {
        Ok(())
    }
    /// 切换身份
    pub async fn query_identity_by_identity(
        password: &String,
        identity: &String,
    ) -> Result<IdentityEntity> {
        ExternalApiIdentity::query_identity_by_identity(identity).await
    }
    pub async fn query_identity_by_public_key(public_key: &String) -> Result<IdentityEntity> {
        ExternalApiIdentity::query_identity_by_public_key(public_key).await
    }
    /// 获取身份列表
    pub async fn identities(password: &String) -> Result<Vec<IdentityEntity>> {
        ExternalApiIdentity::identities().await
    }
    /// 获取身份列表
    pub async fn create_identity(
        password: &String,
        create_request: &IdentityCreateRequest,
        hash: &String,
    ) -> Result<()> {
        ExternalApiIdentity::create_identity(create_request, hash).await
    }
    /// 获取身份列表
    pub async fn remove_identity(password: &String, identity: &String) -> Result<()> {
        ExternalApiIdentity::remove_identity(identity).await
    }
    /// 获取身份列表
    pub async fn update_identity(password: &String, request: &IdentityUpdateRequest) -> Result<()> {
        ExternalApiIdentity::update_identity(request).await
    }
}
