use crate::chain_api::identity::ExternalApiIdentity;
use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{IdentityEntity, ListIdentitiesResponse};

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
    /// 获取身份列表
    pub async fn identities(public_key: &String) -> Result<Vec<IdentityEntity>> {
        ExternalApiIdentity::identities(public_key).await
    }
}
