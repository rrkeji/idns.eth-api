use anyhow::{anyhow, Context, Result};
use hyper::{service::Service, Body, Client, Request};
use idns_eth_api::idns::identity::{
    IdentityCreateRequest, IdentityEntity, IdentityUpdateRequest, ListIdentitiesResponse,
};

use crate::constants::IDENTITY_URL;

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
    pub ed25519_ciphertext: String,
    pub ed25519_salt: String,
    pub ed25519_nonce: String,
    pub sr25519_public_key: String,
    pub sr25519_ciphertext: String,
    pub sr25519_salt: String,
    pub sr25519_nonce: String,
    pub create_at: u64,
}

pub struct ExternalApiIdentity {}

impl ExternalApiIdentity {
    //"public_key""signature""nonce"
    /// 获取身份列表
    pub async fn identities() -> Result<Vec<IdentityEntity>> {
        let (public_key, _, _) = crate::get_external_api_identity_signature()?;

        //访问
        if let Ok(json_str) =
            identity_get_request(format!("/identity/list/{}", public_key).as_str()).await
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
                    ed25519_ciphertext: item.ed25519_ciphertext.clone(),
                    ed25519_salt: item.ed25519_salt.clone(),
                    ed25519_nonce: item.ed25519_nonce.clone(),
                    sr25519_public_key: item.sr25519_public_key.clone(),
                    sr25519_ciphertext: item.sr25519_ciphertext.clone(),
                    sr25519_salt: item.sr25519_salt.clone(),
                    sr25519_nonce: item.sr25519_nonce.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 查询一个
    pub async fn query_identity_by_identity(identity: &String) -> Result<IdentityEntity> {
        //访问
        if let Ok(json_str) =
            identity_get_request(format!("/identity/get/{}", identity).as_str()).await
        {
            //
            let item: IdentityJsonEntity = serde_json::from_str(&json_str)?;
            Ok(IdentityEntity {
                identity: item.identity.clone(),
                public_key: item.account_public_key.clone(),
                name: item.name.clone(),
                avatar: item.avatar.clone(),
                identity_type: item.identity_type.clone(),
                description: item.description.clone(),
                cid: item.cid.clone(),
                ed25519_ciphertext: item.ed25519_ciphertext.clone(),
                ed25519_salt: item.ed25519_salt.clone(),
                ed25519_nonce: item.ed25519_nonce.clone(),
                sr25519_public_key: item.sr25519_public_key.clone(),
                sr25519_ciphertext: item.sr25519_ciphertext.clone(),
                sr25519_salt: item.sr25519_salt.clone(),
                sr25519_nonce: item.sr25519_nonce.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 查询一个
    pub async fn query_identity_by_public_key(public_key: &String) -> Result<IdentityEntity> {
        //访问
        if let Ok(json_str) =
            identity_get_request(format!("/identity/getbypk/{}", public_key).as_str()).await
        {
            //
            let item: IdentityJsonEntity = serde_json::from_str(&json_str)?;
            Ok(IdentityEntity {
                identity: item.identity.clone(),
                public_key: item.account_public_key.clone(),
                name: item.name.clone(),
                avatar: item.avatar.clone(),
                identity_type: item.identity_type.clone(),
                description: item.description.clone(),
                cid: item.cid.clone(),
                ed25519_ciphertext: item.ed25519_ciphertext.clone(),
                ed25519_salt: item.ed25519_salt.clone(),
                ed25519_nonce: item.ed25519_nonce.clone(),
                sr25519_public_key: item.sr25519_public_key.clone(),
                sr25519_ciphertext: item.sr25519_ciphertext.clone(),
                sr25519_salt: item.sr25519_salt.clone(),
                sr25519_nonce: item.sr25519_nonce.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 创建身份
    pub async fn create_identity(request: &IdentityCreateRequest, hash: &String) -> Result<()> {
        let body = &json!({
            "name": request.name.clone(),
            "avatar":request.avatar.clone(),
            "identity_type":request.identity_type.clone(),
            "description":request.description.clone(),
            "hash": hash.clone()
        })
        .to_string();

        //
        if let Ok(res) =
            identity_post_request("/identity/create", "application/json;charset=UTF-8", body)
                .await
                .with_context(|| format!("创建身份请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建身份 {}!", body))
        }
    }
    /// 删除身份
    pub async fn remove_identity(identity: &String) -> Result<()> {
        //访问
        if let Ok(_) = identity_get_request(format!("/identity/remove/{}", identity).as_str()).await
        {
            //
            Ok(())
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 更新身份
    pub async fn update_identity(request: &IdentityUpdateRequest) -> Result<()> {
        let body = &json!({
            "identity":request.identity.clone(),
            "name": request.name.clone(),
            "avatar":request.avatar.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) =
            identity_post_request("/identity/update", "application/json;charset=UTF-8", body)
                .await
                .with_context(|| format!("创建身份请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建身份 {}!", body))
        }
    }
}

/// 内部请求的方法
pub async fn identity_get_request(path: &str) -> Result<String> {
    //请求地址
    let uri = format!("{}{}", IDENTITY_URL, path).parse::<http::Uri>()?;

    let mut client = Client::new();
    let body = Body::empty();

    let (public_key, signature, nonce) = crate::get_external_api_identity_signature()?;

    let request = Request::get(uri)
        .header("idns-pub-key", public_key)
        .header("idns-signature", signature)
        .header("idns-nonce", nonce)
        .body(body)?;
    //发送请求
    let mut resp = client.call(request).await?;

    //获取版本的信息resp.headers()
    tracing::debug!("Response: {:?}", resp.status());
    tracing::debug!("Headers: {:?}", resp.headers());
    //
    let result = hyper::body::to_bytes(resp).await;
    match result {
        Ok(plain) => {
            tracing::debug!("response plain: {:?}", plain);
            Ok(String::from_utf8(plain[..].to_vec())?)
        }
        Err(err) => Err(anyhow!("Fail request {}!", err)),
    }
}

/// 内部请求的方法
pub(crate) async fn identity_post_request(
    path: &str,
    content_type: &str,
    content: &String,
) -> Result<String> {
    //请求地址
    let uri = format!("{}{}", IDENTITY_URL, path).parse::<http::Uri>()?;

    let mut client = Client::new();
    let body = Body::from(content.clone());
    let (public_key, signature, nonce) = crate::get_external_api_identity_signature()?;
    let request = Request::post(uri)
        .header("idns-pub-key", public_key)
        .header("idns-signature", signature)
        .header("idns-nonce", nonce)
        .header("content-type", content_type)
        .body(body)?;
    // 发送请求
    let mut resp = client.call(request).await?;

    tracing::debug!("Response: {:?}", resp.status());
    tracing::debug!("Headers: {:?}", resp.headers());

    let result = hyper::body::to_bytes(resp).await;
    match result {
        Ok(plain) => {
            tracing::debug!("response plain: {:?}", plain);
            Ok(String::from_utf8(plain[..].to_vec())?)
        }
        Err(err) => Err(anyhow!("Fail request {}!", err)),
    }
}
