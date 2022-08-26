use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{ListVerifiableCredentialsResponse, VerifiableCredentialEntity};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct VerifiableCredentialJsonEntity {
    pub source_identity: String,
    pub holder_identity: String,
    pub meta_credential_name: String,
    pub cid: String,
    pub description: String,
    pub create_at: u64,
}

pub struct ExternalApiVerifiableCredential {}

impl ExternalApiVerifiableCredential {
    /// 获取属性值列表
    pub async fn verifiable_credential_list_by_source(
        source_identity: &String,
    ) -> Result<Vec<VerifiableCredentialEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/verifiable_credential/listbysource/{}", source_identity).as_str(),
        )
        .await
        {
            //
            let list: Vec<VerifiableCredentialJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| VerifiableCredentialEntity {
                    source_identity: item.source_identity.clone(),
                    holder_identity: item.holder_identity.clone(),
                    meta_credential_name: item.meta_credential_name.clone(),
                    cid: item.cid.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 获取属性值列表
    pub async fn verifiable_credential_list_by_holder(
        holder_identity: &String,
    ) -> Result<Vec<VerifiableCredentialEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/verifiable_credential/listbyholder/{}", holder_identity).as_str(),
        )
        .await
        {
            //
            let list: Vec<VerifiableCredentialJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| VerifiableCredentialEntity {
                    source_identity: item.source_identity.clone(),
                    holder_identity: item.holder_identity.clone(),
                    meta_credential_name: item.meta_credential_name.clone(),
                    cid: item.cid.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 查询一个
    pub async fn query_one_verifiable_credential(
        id: &String,
    ) -> Result<VerifiableCredentialEntity> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/verifiable_credential/get/{}", id).as_str(),
        )
        .await
        {
            //
            let item: VerifiableCredentialJsonEntity = serde_json::from_str(&json_str)?;
            Ok(VerifiableCredentialEntity {
                source_identity: item.source_identity.clone(),
                holder_identity: item.holder_identity.clone(),
                meta_credential_name: item.meta_credential_name.clone(),
                cid: item.cid.clone(),
                description: item.description.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
}
