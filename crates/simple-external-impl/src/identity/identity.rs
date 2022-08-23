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
    pub ciphertext: String,
    pub salt: String,
    pub nonce: String,
    pub create_at: u64,
}

pub struct ExternalApiIdentity {
    pub public_key: String,
    pub signature: String,
    pub nonce: String,
}

impl ExternalApiIdentity {
    //"public_key""signature""nonce"
    /// 获取身份列表
    pub async fn identities(&self) -> Result<Vec<IdentityEntity>> {
        let public_key = self.public_key.clone();

        //访问
        if let Ok(json_str) = self
            .identity_get_request(format!("/identity/list/{}", public_key).as_str())
            .await
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
    /// 查询一个
    pub async fn query_identity_by_identity(&self, identity: &String) -> Result<IdentityEntity> {
        //访问
        if let Ok(json_str) = self
            .identity_get_request(format!("/identity/get/{}", identity).as_str())
            .await
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
                ciphertext: item.ciphertext.clone(),
                salt: item.salt.clone(),
                nonce: item.nonce.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 创建身份
    pub async fn create_identity(
        &self,
        create_request: &IdentityCreateRequest,
        hash: &String,
    ) -> Result<()> {
        //

        Ok(())
    }
    /// 删除身份
    pub async fn remove_identity(&self, identity: &String) -> Result<()> {
        Ok(())
    }
    /// 更新身份
    pub async fn update_identity(
        &self,
        create_request: &IdentityUpdateRequest,
        hash: &String,
    ) -> Result<()> {
        Ok(())
    }
}

impl ExternalApiIdentity {
    /// 内部请求的方法
    pub(crate) async fn identity_get_request(&self, path: &str) -> Result<String> {
        //请求地址
        let uri = format!("{}{}", IDENTITY_URL, path).parse::<http::Uri>()?;

        let mut client = Client::new();
        let body = Body::empty();

        let request = Request::get(uri)
            .header(
                "idns-pub-key",
                "7a0b9c92b65218204416d335c7b85ef9d47da1ba92bbb2b3a23224c6cd38ce54",
            )
            .header("idns-signature", "signature")
            .header("idns-nonce", "nonce")
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
        &self,
        path: &str,
        content_type: &str,
        content: &String,
    ) -> Result<String> {
        //请求地址
        let uri = format!("{}{}", IDENTITY_URL, path).parse::<http::Uri>()?;

        let mut client = Client::new();
        let body = Body::from(content.clone());

        let request = Request::post(uri)
            .header(
                "idns-pub-key",
                "7a0b9c92b65218204416d335c7b85ef9d47da1ba92bbb2b3a23224c6cd38ce54",
            )
            .header("idns-signature", "signature")
            .header("idns-nonce", "nonce")
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
}
