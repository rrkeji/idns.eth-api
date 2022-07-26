use anyhow::{anyhow, Result};
use hyper::{service::Service, Body, Client, Request};

pub(crate) const KVSTORE_URL: &'static str = "http://49.232.102.140:35090";

pub struct KVStore {}

/// KVStore
impl KVStore {
    /// 通过账户的public_key获取值, token为可选项
    ///
    /// account_public_key 账户public Key
    ///
    /// 返回
    pub async fn get_value(account_public_key: &str) -> Result<String> {
        //
        _kvstore_get_request(format!("/kv/get/{}", account_public_key).as_str(), None).await
    }
    /// 设置值, token必须传入
    pub async fn set_value(value: &str, token: &str) -> Result<()> {
        //
        _kvstore_get_request(format!("/kv/set/{}", value).as_str(), Some(token)).await?;
        Ok(())
    }

    /// 通过账户的public_key获取该账户某个应用下的值, token为可选项
    pub async fn get_application_value(
        account_public_key: &str,
        application_public_key: &str,
        token: Option<&str>,
    ) -> Result<String> {
        //
        _kvstore_get_request(
            format!("/kv/aget/{}/{}", application_public_key, account_public_key).as_str(),
            token,
        )
        .await
    }
    /// 设置该账号下某个应用的值, token必须传入
    pub async fn set_application_value(
        account_public_key: &str,
        value: &str,
        application_public_key: &str,
        token: &str,
    ) -> Result<()> {
        //
        _kvstore_get_request(
            format!(
                "/kv/aset/{}/{}/{}",
                application_public_key, account_public_key, value
            )
            .as_str(),
            Some(token),
        )
        .await?;
        Ok(())
    }
}

/// 内部请求的方法
pub(crate) async fn _kvstore_get_request(path: &str, token: Option<&str>) -> Result<String> {
    //请求地址
    let uri = format!("{}{}", KVSTORE_URL, path)
        .parse::<http::Uri>()
        .unwrap();

    let mut client = Client::new();
    let body = Body::empty();

    let request = if let Some(token_value) = token {
        Request::get(uri)
            .header("Authorization", token_value)
            .body(body)
            .unwrap()
    } else {
        Request::get(uri).body(body).unwrap()
    };
    //发送请求
    let mut resp = client.call(request).await.unwrap();

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

        let token = aw!(login(public_key, phrase))
            .with_context(|| format!("Failed to login from {}", public_key))
            .unwrap();
        // KVStore::set_value(phrase, token);
        assert_eq!(KVStore::get_value(public_key), phrase);
    }
}
