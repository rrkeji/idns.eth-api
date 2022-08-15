use std::mem::MaybeUninit;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use parking_lot::RwLock;
use tokio::io::BufReader;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, watch};
use tokio::{sync, time};

use crate::common::cipher::Aes128Ctr;
use crate::common::net::msg_operator::{TcpMsgReader, TcpMsgWriter, UdpMsgSocket, TCP_BUFF_SIZE};
use crate::common::net::proto::{HeartbeatType, MsgResult, Node, NodeId, TcpMsg, UdpMsg};
use crate::common::net::SocketExt;
use crate::common::{HashMap, MapInit};
use crate::{Listener, ServerConfigFinalize};

use crate::{node::NodeDb, tunnel::tunnel};

pub async fn tcp_handler(
    listen_addr: SocketAddr,
    key: Aes128Ctr,
    node_db: Arc<NodeDb>,
) -> Result<()> {
    let listener = TcpListener::bind(listen_addr)
        .await
        .with_context(|| format!("TCP socket bind {} error", listen_addr))?;

    info!("TCP socket listening on {}", listen_addr);

    loop {
        let key = key.clone();
        let node_db = node_db.clone();
        let (stream, peer_addr) = listener.accept().await.context("Accept connection error")?;

        tokio::spawn(async move {
            if let Err(e) = tunnel(stream, key, node_db).await {
                error!("Peer addr {} tunnel error -> {:?}", peer_addr, e)
            }
        });
    }
}
