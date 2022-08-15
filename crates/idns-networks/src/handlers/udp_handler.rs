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

use crate::{
    node::{NodeDb, NodeHandle},
    tunnel::tunnel,
};

pub async fn udp_handler(
    listen_addr: SocketAddr,
    key: Aes128Ctr,
    node_db: Arc<NodeDb>,
) -> Result<()> {
    let socket = UdpSocket::bind(listen_addr)
        .await
        .with_context(|| format!("UDP socket bind {} error", listen_addr))?;

    info!("UDP socket listening on {}", listen_addr);

    let mut msg_socket = UdpMsgSocket::new(&socket, key);

    loop {
        let msg = match msg_socket.read().await {
            Ok(v) => v,
            Err(e) => {
                error!("UDP msg recv error -> {:?}", e);
                continue;
            }
        };

        match msg {
            (UdpMsg::Heartbeat(node_id, seq, HeartbeatType::Req), peer_addr) => {
                let heartbeat = UdpMsg::Heartbeat(node_id, seq, HeartbeatType::Resp);
                msg_socket.write(&heartbeat, peer_addr).await?;

                let res = node_db.get(&node_id, |v| match v {
                    Some(NodeHandle { node, .. }) => {
                        !matches!(node.wan_udp_addr, Some(addr) if addr == peer_addr)
                    }
                    None => false,
                });

                if !res {
                    continue;
                }

                node_db.get_mut(&node_id, |v| {
                    if let Some(NodeHandle { node, .. }) = v {
                        debug!("Update {} wan_udp_addr to {}", node_id, peer_addr);
                        node.wan_udp_addr = Some(peer_addr)
                    }
                })?;
            }
            _ => error!("Invalid UDP msg"),
        };
    }
}
