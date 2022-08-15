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

use crate::node::{Bridge, NodeDb, NodeHandle};

pub async fn tunnel(mut stream: TcpStream, key: Aes128Ctr, node_db: Arc<NodeDb>) -> Result<()> {
    stream.set_keepalive()?;
    let (rx, mut tx) = stream.split();
    let mut rx = BufReader::with_capacity(TCP_BUFF_SIZE, rx);

    let mut rx_key = key.clone();
    let mut tx_key = key;

    let mut msg_reader = TcpMsgReader::new(&mut rx, &mut rx_key);
    let mut msg_writer = TcpMsgWriter::new(&mut tx, &mut tx_key);

    let msg = msg_reader.read().await?;
    let mut bridge = match msg {
        TcpMsg::Register(node) => {
            let register_time = node.register_time;
            let remain = Utc::now().timestamp() - register_time;

            if (remain > 10) || (remain < -10) {
                msg_writer
                    .write(&TcpMsg::Result(MsgResult::Timeout))
                    .await?;
                return Err(anyhow!("Register message timeout"));
            }

            msg_writer
                .write(&TcpMsg::Result(MsgResult::Success))
                .await?;
            node_db.insert(node)?
        }
        _ => return Err(anyhow!("Register error")),
    };

    let node_id = bridge.node.id;
    let inner_node_db = &*node_db;

    let (tx, mut rx) = sync::mpsc::unbounded_channel::<TcpMsg>();

    let latest_recv_heartbeat_time = AtomicI64::new(Utc::now().timestamp());
    let seq: AtomicU32 = AtomicU32::new(0);

    let fut1 = async {
        loop {
            match msg_reader.read().await? {
                TcpMsg::Forward(data, dest_node_id) => {
                    inner_node_db.get(&dest_node_id, |op| {
                        if let Some(NodeHandle { tx: channel_tx, .. }) = op {
                            if let Err(TrySendError::Closed(_)) =
                                channel_tx.try_send((data.into(), node_id))
                            {
                                error!("Dest node {} channel closed", dest_node_id)
                            }
                        }
                    });
                }
                TcpMsg::Heartbeat(seq, HeartbeatType::Req) => {
                    let heartbeat = TcpMsg::Heartbeat(seq, HeartbeatType::Resp);
                    tx.send(heartbeat).map_err(|e| anyhow!(e.to_string()))?;
                }
                TcpMsg::Heartbeat(recv_seq, HeartbeatType::Resp) => {
                    if seq.load(Ordering::Relaxed) == recv_seq {
                        latest_recv_heartbeat_time.store(Utc::now().timestamp(), Ordering::Relaxed)
                    }
                }
                _ => return Result::<()>::Err(anyhow!("Invalid TCP msg")),
            }
        }
    };

    let fut2 = async {
        let mut heartbeat_interval =
            time::interval(crate::server::get_config().tcp_heartbeat_interval);
        let mut check_heartbeat_timeout = time::interval(Duration::from_secs(30));

        loop {
            tokio::select! {
                 opt = rx.recv() => {
                     match opt {
                        Some(heartbeat) => msg_writer.write(&heartbeat).await?,
                        None => return Ok(())
                    }
                }
                res = bridge.watch_rx.changed() => {
                    res?;
                    let node_list = bridge.watch_rx.borrow().clone();
                    let msg = TcpMsg::NodeMap(node_list);
                    msg_writer.write(&msg).await?;
                }
                res = bridge.channel_rx.recv() => {
                    let (data, src_node_id) = res.ok_or_else(|| anyhow!("Node {} channel closed", node_id))?;
                    let msg = TcpMsg::Forward(&data, src_node_id);
                    msg_writer.write(&msg).await?;
                    debug!("Forward packet {} to {}", src_node_id, node_id)
                }
                _ = heartbeat_interval.tick() => {
                    seq.fetch_add(1, Ordering::Relaxed);
                    let heartbeat = TcpMsg::Heartbeat(seq.load(Ordering::Relaxed), HeartbeatType::Req);
                    msg_writer.write(&heartbeat).await?;
                }
                _ = check_heartbeat_timeout.tick() => {
                     if Utc::now().timestamp() - latest_recv_heartbeat_time.load(Ordering::Relaxed) > 30 {
                        return Err(anyhow!("Heartbeat recv timeout"))
                    }
                }
            }
        }
    };

    tokio::try_join!(fut1, fut2)?;
    Ok(())
}
