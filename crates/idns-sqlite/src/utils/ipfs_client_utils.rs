use crate::ipfs_api::{IpfsApi, IpfsClient};
use anyhow::{anyhow, Result};
use bytes::{BufMut, BytesMut};
use futures::stream::{FuturesUnordered, StreamExt, TryStreamExt};
use std::{convert::TryFrom, io::Cursor};
use tokio::runtime::Handle;

pub fn get_ipfs_client() -> Result<IpfsClient> {
    //
    let res = IpfsClient::default();

    Ok(res)
}

/// 存储值，并返回内容ID
pub async fn ipfs_add_content(value: Vec<u8>) -> Result<String> {
    //
    //保存到到IPFS
    let data = Cursor::new(value);

    let client = get_ipfs_client()?;

    if let Ok(res_result) = client.add(data).await {
        tracing::debug!("保存IPFS成功:{:#?}", res_result);
        Ok(res_result.hash)
    } else {
        Err(anyhow!(""))
    }
}

pub async fn ipfs_get_content(cid: &String) -> Result<Vec<u8>> {
    //
    let client = get_ipfs_client()?;
    //读取数据
    let mut stream = client.cat(cid.as_str());
    //
    let mut buf = BytesMut::with_capacity(40960);

    while let Some(parts) = stream.next().await {
        // bytes.
        if let Ok(bs) = parts {
            buf.put(bs);
        }
    }
    Ok(buf.to_vec())
}
