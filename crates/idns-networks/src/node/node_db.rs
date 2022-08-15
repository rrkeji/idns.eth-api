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

pub struct NodeHandle {
    pub node: Node,
    pub tx: Sender<(Box<[u8]>, NodeId)>,
}

pub struct Bridge<'a> {
    pub node: Node,
    pub channel_rx: Receiver<(Box<[u8]>, NodeId)>,
    pub watch_rx: watch::Receiver<HashMap<NodeId, Node>>,
    pub node_db: &'a NodeDb,
}

pub struct NodeDb {
    pub mapping: RwLock<HashMap<NodeId, NodeHandle>>,
    pub watch: (
        watch::Sender<HashMap<NodeId, Node>>,
        watch::Receiver<HashMap<NodeId, Node>>,
    ),
}

impl Drop for Bridge<'_> {
    fn drop(&mut self) {
        let node_id = self.node.id;
        let register_time = self.node.register_time;

        let mut guard = self.node_db.mapping.write();

        if let Some(NodeHandle { node, .. }) = guard.get(&node_id) {
            if node.register_time == register_time {
                guard.remove(&node_id);
                drop(guard);

                if let Err(e) = self.node_db.sync() {
                    error!("Sync node db error: {:?}", e)
                }
            }
        };
    }
}

impl NodeDb {
    pub fn new() -> Self {
        NodeDb {
            mapping: RwLock::new(HashMap::new()),
            watch: watch::channel(HashMap::new()),
        }
    }

    pub fn insert(&self, node: Node) -> Result<Bridge> {
        let node_id = node.id;
        let (_, watch_rx) = &self.watch;
        let (tx, rx) =
            mpsc::channel::<(Box<[u8]>, NodeId)>(crate::server::get_config().channel_limit);

        self.mapping.write().insert(
            node_id,
            NodeHandle {
                node: node.clone(),
                tx,
            },
        );
        self.sync()?;

        let bridge = Bridge {
            node,
            channel_rx: rx,
            watch_rx: watch_rx.clone(),
            node_db: self,
        };
        Ok(bridge)
    }

    pub fn get<R, F: FnOnce(Option<&NodeHandle>) -> R>(&self, id: &NodeId, f: F) -> R {
        f(self.mapping.read().get(id))
    }

    pub fn get_mut<R, F: FnOnce(Option<&mut NodeHandle>) -> R>(
        &self,
        id: &NodeId,
        f: F,
    ) -> Result<R> {
        let r = f(self.mapping.write().get_mut(id));
        self.sync()?;
        Ok(r)
    }

    pub fn sync(&self) -> Result<()> {
        let (tx, _) = &self.watch;

        let node_list: HashMap<NodeId, Node> = self
            .mapping
            .read()
            .iter()
            .map(|(node_id, handle)| (*node_id, handle.node.clone()))
            .collect();

        tx.send(node_list).map_err(|_| anyhow!("Sync error"))?;
        Ok(())
    }
}
