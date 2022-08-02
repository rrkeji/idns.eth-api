use anyhow::{anyhow, Result};
use hyper::{service::Service, Body, Client, Request};

use crate::account::IdnsToken;

pub(crate) const KVSTORE_URL: &'static str = "http://49.232.102.140:35090";
// pub(crate) const KVSTORE_URL: &'static str = "http://127.0.0.1:35090";

pub struct KVStore {}

/// KVStore
impl KVStore {
    /// 通过账户的public_key获取值, token为可选项
    ///
    /// account_public_key 账户public Key
    ///
    /// 返回
    pub async fn get_value(token: &IdnsToken) -> Result<(String, i64)> {
        let public_key = token.public_key.clone();
        let application_key = token.application_key.clone();
        let token_option = token.token.clone();

        KVStore::get_application_value(
            public_key.as_str(),
            application_key.as_str(),
            if token_option == "" {
                None
            } else {
                Some(token_option.clone())
            },
        )
        .await
    }
    /// 设置值, token必须传入
    pub async fn set_value(token: &IdnsToken, old_version: i64, value: &str) -> Result<i64> {
        let token_option = token.token.clone();
        //内部应用
        let (_, version) = _kvstore_get_request(
            format!("/kv/set/{}", value).as_str(),
            Some(token_option.clone()),
            old_version,
        )
        .await?;
        Ok(version)
    }

    /// 通过账户的public_key获取该账户某个应用下的值, token为可选项
    pub async fn get_application_value(
        account_public_key: &str,
        application_public_key: &str,
        token: Option<String>,
    ) -> Result<(String, i64)> {
        //
        _kvstore_get_request(
            format!("/kv/aget/{}/{}", application_public_key, account_public_key).as_str(),
            token,
            0i64,
        )
        .await
    }
}

/// 内部请求的方法
pub(crate) async fn _kvstore_get_request(
    path: &str,
    token: Option<String>,
    version: i64,
) -> Result<(String, i64)> {
    //请求地址
    let uri = format!("{}{}", KVSTORE_URL, path).parse::<http::Uri>()?;

    let mut client = Client::new();
    let body = Body::empty();

    let request = if let Some(token_value) = token {
        Request::get(uri)
            .header("Authorization", token_value)
            .header("idns-kv-version", version)
            .body(body)?
    } else {
        Request::get(uri)
            .header("idns-kv-version", version)
            .body(body)?
    };
    //发送请求
    let mut resp = client.call(request).await?;

    let mut version: i64 = -1;
    //获取版本的信息resp.headers()
    if resp.headers().contains_key("idns-kv-version") {
        if let Some(kv_version) = resp.headers().get("idns-kv-version") {
            version = match kv_version.to_str() {
                Ok(ver_str) => {
                    //
                    ver_str.parse::<i64>()?
                }
                _ => 0,
            }
        } else {
            version = 0;
        }
    }

    tracing::debug!("Response: {:?}", resp.status());
    tracing::debug!("Headers: {:?}", resp.headers());
    //
    let result = hyper::body::to_bytes(resp).await;
    match result {
        Ok(plain) => {
            tracing::debug!("response plain: {:?}", plain);
            Ok((String::from_utf8(plain[..].to_vec())?, version))
        }
        Err(err) => Err(anyhow!("Fail request {}!", err)),
    }
}

/// 内部请求的方法
pub(crate) async fn _kvstore_post_request(
    path: &str,
    content_type: &str,
    content: &String,
    token: Option<String>,
) -> Result<String> {
    //请求地址
    let uri = format!("{}{}", KVSTORE_URL, path).parse::<http::Uri>()?;

    let mut client = Client::new();
    let body = Body::from(content.clone());

    let request = if let Some(token_value) = token {
        Request::post(uri)
            .header("Authorization", token_value)
            .header("content-type", content_type)
            .body(body)?
    } else {
        Request::post(uri)
            .header("content-type", content_type)
            .body(body)?
    };
    //发送请求
    let mut resp = client.call(request).await?;

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

//
#[cfg(test)]
mod tests {
    use super::*;

    use crate::account::{login, ALICE_PHRASE, ALICE_PUBLIC_KEY};
    use anyhow::Context;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn get_value() {
        let public_key = ALICE_PUBLIC_KEY;
        let phrase = ALICE_PHRASE;

        // let token = aw!(login(public_key, phrase))
        //     .with_context(|| format!("Failed to login from {}", public_key))?;
        // KVStore::set_value(phrase, token);
        // assert_eq!(KVStore::get_value(public_key), phrase);
    }
}
