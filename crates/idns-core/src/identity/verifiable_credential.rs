use crate::chain_api::identity::ExternalApiVerifiableCredential;
use anyhow::{anyhow, Context, Result};

use idns_eth_api::idns::identity::VerifiableCredentialEntity;
pub struct IdnsVerifiableCredential {}

impl IdnsVerifiableCredential {
    /// 切换身份
    pub async fn query_one_verifiable_credential(
        id: &String,
    ) -> Result<VerifiableCredentialEntity> {
        ExternalApiVerifiableCredential::query_one_verifiable_credential(id).await
    }
    /// 获取身份列表
    pub async fn verifiable_credential_list_by_holder(
        holder_identity: &String,
    ) -> Result<Vec<VerifiableCredentialEntity>> {
        ExternalApiVerifiableCredential::verifiable_credential_list_by_holder(holder_identity).await
    }
    /// 获取身份列表
    pub async fn verifiable_credential_list_by_source(
        source_identity: &String,
    ) -> Result<Vec<VerifiableCredentialEntity>> {
        ExternalApiVerifiableCredential::verifiable_credential_list_by_source(source_identity).await
    }
}
