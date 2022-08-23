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
        let user_phrase = crate::get_user_phrase(password)?;
        let body = crate::get_signature_nonce(&user_phrase)?;
        ExternalApiIdentity {
            public_key: body.0,
            signature: body.1,
            nonce: body.2,
        }
        .query_identity_by_identity(identity)
        .await
    }
    /// 获取身份列表
    pub async fn identities(password: &String) -> Result<Vec<IdentityEntity>> {
        let user_phrase = crate::get_user_phrase(password)?;
        let body = crate::get_signature_nonce(&user_phrase)?;
        ExternalApiIdentity {
            public_key: body.0,
            signature: body.1,
            nonce: body.2,
        }
        .identities()
        .await
    }
    /// 获取身份列表
    pub async fn create_identity(
        password: &String,
        create_request: &IdentityCreateRequest,
        hash: &String,
    ) -> Result<Vec<IdentityEntity>> {
        let user_phrase = crate::get_user_phrase(password)?;
        let body = crate::get_signature_nonce(&user_phrase)?;
        ExternalApiIdentity {
            public_key: body.0,
            signature: body.1,
            nonce: body.2,
        }
        .create_identity(create_request, hash)
        .await
    }
    /// 获取身份列表
    pub async fn remove_identity(
        password: &String,
        identity: &String,
    ) -> Result<Vec<IdentityEntity>> {
        let user_phrase = crate::get_user_phrase(password)?;
        let body = crate::get_signature_nonce(&user_phrase)?;
        ExternalApiIdentity {
            public_key: body.0,
            signature: body.1,
            nonce: body.2,
        }
        .remove_identity(password, identity)
        .await
    }
    /// 获取身份列表
    pub async fn update_identity(
        password: &String,
        request: &IdentityUpdateRequest,
        hash: &String,
    ) -> Result<Vec<IdentityEntity>> {
        let user_phrase = crate::get_user_phrase(password)?;
        let body = crate::get_signature_nonce(&user_phrase)?;
        ExternalApiIdentity {
            public_key: body.0,
            signature: body.1,
            nonce: body.2,
        }
        .update_identity(request, hash)
        .await
    }
}
