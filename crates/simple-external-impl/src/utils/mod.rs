use anyhow::{anyhow, Result};
use hyper::{service::Service, Body, Client, Request};

use crate::constants::IDENTITY_URL;

/// 内部请求的方法
pub(crate) async fn identity_get_request(path: &str, token: Option<String>) -> Result<String> {
    //请求地址
    let uri = format!("{}{}", IDENTITY_URL, path).parse::<http::Uri>()?;

    let mut client = Client::new();
    let body = Body::empty();

    let request = if let Some(token_value) = token {
        Request::get(uri)
            .header("Authorization", token_value)
            .body(body)?
    } else {
        Request::get(uri).body(body)?
    };
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
    token: Option<String>,
) -> Result<String> {
    //请求地址
    let uri = format!("{}{}", IDENTITY_URL, path).parse::<http::Uri>()?;

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
