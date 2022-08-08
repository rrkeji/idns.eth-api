use crate::ipfs_api::{IpfsApi, IpfsClient};
use anyhow::{anyhow, Result};
use bytes::{BufMut, BytesMut};
use futures::stream::StreamExt;
use std::io::Cursor;
use tokio::runtime::Handle;

pub fn get_ipfs_client() -> Result<IpfsClient> {
    //
    let res = IpfsClient::default();

    Ok(res)
}

/// 存储值，并返回内容ID
pub fn ipfs_add_content(value: Vec<u8>) -> Result<String> {
    //
    //保存到到IPFS
    let data = Cursor::new(value);

    let handle = Handle::current();
    let handle_std = std::thread::spawn(move || {
        handle.block_on(async move {
            if let Ok(client) = get_ipfs_client() {
                let res_result = client.add(data).await;
                match res_result {
                    Ok(res) => {
                        tracing::debug!("保存IPFS成功:{:#?}", res);
                        Ok(res.hash)
                    }
                    Err(e) => {
                        tracing::error!("保存到IPFS失败:{:#?}", e);
                        Err(anyhow!(""))
                    }
                }
            } else {
                Err(anyhow!(""))
            }
        })
    });

    if let Ok(res) = handle_std.join() {
        res
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
