use hbb_common::config::LocalConfig;
use hbb_common::rendezvous_proto::ConnType;
use idns_eth_devices::*;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let mut options = crate::ipc::get_options_async().await;
    let mut key = options.remove("key").unwrap_or("".to_owned());
    let token = LocalConfig::get_option("access_token");
    if key.is_empty() {
        key = crate::platform::get_license_key();
    }
    match idns_eth_devices::client::Client::start("334216107", &key, &token, ConnType::default())
        .await
    {
        Ok((mut peer, direct)) => {
            println!("---");
            loop {
                tokio::select! {
                    res = peer.next() => {
                        if let Some(res) = res {
                            match res {
                                Err(err) => {
                                    println!("Connection closed: {}", err);
                                    break;
                                }
                                Ok(ref bytes) => {
                                    println!("bytes: {:?}", bytes);
                                }
                            }
                        } else {
                            println!("None1");
                            break;
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("---11111:{}", err);
        }
    }

    let ten_millis = time::Duration::from_millis(1000000);
    thread::sleep(ten_millis);
}
