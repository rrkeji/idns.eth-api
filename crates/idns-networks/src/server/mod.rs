use std::mem::MaybeUninit;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};

use crate::common::cipher::Aes128Ctr;
use crate::{Listener, ServerConfigFinalize};

static mut CONFIG: MaybeUninit<ServerConfigFinalize> = MaybeUninit::uninit();

use crate::handlers::{tcp_handler, udp_handler};
use crate::node::NodeDb;

pub fn set_config(config: ServerConfigFinalize) {
    unsafe { CONFIG.write(config) };
}

pub fn get_config() -> &'static ServerConfigFinalize {
    unsafe { CONFIG.assume_init_ref() }
}

pub(super) async fn start(server_config: ServerConfigFinalize) {
    set_config(server_config);
    let mut list = Vec::with_capacity(get_config().listeners.len());

    for Listener { listen_addr, key } in &get_config().listeners {
        let handle = async move {
            let key = Aes128Ctr::new(key.as_bytes());
            let node_db = Arc::new(NodeDb::new());
            let key_ref = key.clone();
            let node_db_ref = node_db.clone();

            let udp_handle = async {
                tokio::spawn(udp_handler(*listen_addr, key, node_db))
                    .await?
                    .context("UDP handler error")
            };

            let tcp_handle = async {
                tokio::spawn(tcp_handler(*listen_addr, key_ref, node_db_ref))
                    .await?
                    .context("TCP handler error")
            };

            // TODO when error occurs in one handler, the other does not end
            if let Err(e) = tokio::try_join!(udp_handle, tcp_handle) {
                error!("Server {} execute error -> {:?}", listen_addr, e)
            };
        };
        list.push(handle);
    }

    futures_util::future::join_all(list).await;
}
