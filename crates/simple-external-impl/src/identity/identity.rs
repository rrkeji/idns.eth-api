use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{IdentityEntity, ListIdentitiesResponse};

use crate::utils::{identity_get_request, identity_post_request};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct IdentityJsonEntity {
    pub account_public_key: String,
    pub identity: String,
    pub cid: String,
    pub name: String,
    pub avatar: String,
    pub identity_type: String,
    pub description: String,
    pub ciphertext: String,
    pub salt: String,
    pub nonce: String,
    pub create_at: u64,
}

pub struct ExternalApiIdentity {}

impl ExternalApiIdentity {
    /// 获取身份列表
    pub async fn identities(public_key: &String) -> Result<Vec<IdentityEntity>> {
        //访问
        if let Ok(json_str) =
            identity_get_request(format!("/identity/list/{}", public_key).as_str(), None).await
        {
            //
            let list: Vec<IdentityJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| IdentityEntity {
                    identity: item.identity.clone(),
                    public_key: item.account_public_key.clone(),
                    name: item.name.clone(),
                    avatar: item.avatar.clone(),
                    identity_type: item.identity_type.clone(),
                    description: item.description.clone(),
                    cid: item.cid.clone(),
                    ciphertext: item.ciphertext.clone(),
                    salt: item.salt.clone(),
                    nonce: item.nonce.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 创建身份
    pub fn create_identity() -> Result<Vec<String>> {
        Ok(vec![])
    }
    /// 删除身份
    pub fn remove_identity() -> Result<Vec<String>> {
        Ok(vec![])
    }
    /// 更新身份
    pub fn update_identity() -> Result<Vec<String>> {
        Ok(vec![])
    }
}
