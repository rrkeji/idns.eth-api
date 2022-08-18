use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use mimalloc::MiMalloc;
use serde::{de, Deserialize};

use tokio::runtime::Handle;
use tokio::runtime::Runtime;

use crate::vpnc::client::Req;
use crate::vpnc::common::cipher::Aes128Ctr;
use crate::vpnc::common::net::get_interface_addr;
use crate::vpnc::common::net::proto::ProtocolMode;

mod client;
mod common;
mod tun;

#[macro_export]
macro_rules! ternary {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition {
            $_true
        } else {
            $_false
        }
    };
}

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Deserialize, Clone)]
pub struct TunIpAddr {
    pub ip: Ipv4Addr,
    pub netmask: Ipv4Addr,
}

#[derive(Deserialize, Clone)]
pub struct NetworkRange {
    pub server_addr: String,
    pub tun: TunIpAddr,
    pub key: String,
    pub mode: Option<String>,
    pub lan_ip_addr: Option<IpAddr>,
    pub try_send_to_lan_addr: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub struct ClientConfig {
    pub mtu: Option<usize>,
    pub channel_limit: Option<usize>,
    pub api_addr: Option<SocketAddr>,
    pub tcp_heartbeat_interval_secs: Option<u64>,
    pub udp_heartbeat_interval_secs: Option<u64>,
    pub reconnect_interval_secs: Option<u64>,
    pub udp_socket_recv_buffer_size: Option<usize>,
    pub udp_socket_send_buffer_size: Option<usize>,
    pub tun_handler_thread_count: Option<usize>,
    pub udp_handler_thread_count: Option<usize>,
    pub network_ranges: Vec<NetworkRange>,
}

#[derive(Clone)]
pub struct NetworkRangeFinalize {
    pub server_addr: String,
    pub tun: TunIpAddr,
    pub key: Aes128Ctr,
    pub mode: ProtocolMode,
    pub lan_ip_addr: Option<IpAddr>,
    pub try_send_to_lan_addr: bool,
}

#[derive(Clone)]
pub struct ClientConfigFinalize {
    pub mtu: usize,
    pub channel_limit: usize,
    pub api_addr: SocketAddr,
    pub tcp_heartbeat_interval: Duration,
    pub udp_heartbeat_interval: Duration,
    pub reconnect_interval: Duration,
    pub udp_socket_recv_buffer_size: Option<usize>,
    pub udp_socket_send_buffer_size: Option<usize>,
    pub tun_handler_thread_count: usize,
    pub udp_handler_thread_count: usize,
    pub network_ranges: Vec<NetworkRangeFinalize>,
}

impl TryFrom<ClientConfig> for ClientConfigFinalize {
    type Error = anyhow::Error;

    fn try_from(config: ClientConfig) -> Result<Self> {
        let mut ranges = Vec::with_capacity(config.network_ranges.len());

        for range in config.network_ranges {
            let mode = ProtocolMode::from_str(range.mode.as_deref().unwrap_or("UDP_AND_TCP"))?;

            let resolve_server_addr = range
                .server_addr
                .to_socket_addrs()?
                .next()
                .ok_or_else(|| anyhow!("Server host not found"))?;

            let lan_ip_addr = match range.lan_ip_addr {
                None => {
                    if mode.udp_support() {
                        let lan_addr = get_interface_addr(resolve_server_addr)?;
                        Some(lan_addr)
                    } else {
                        None
                    }
                }
                Some(addr) => {
                    if addr.is_loopback() {
                        return Err(anyhow!("LAN address cannot be a loopback address"));
                    }

                    if addr.is_unspecified() {
                        return Err(anyhow!("LAN address cannot be unspecified address"));
                    }
                    Some(addr)
                }
            };

            let range_finalize = NetworkRangeFinalize {
                server_addr: {
                    if resolve_server_addr.ip().is_loopback() {
                        return Err(anyhow!("Server address cannot be a loopback address"));
                    }
                    range.server_addr
                },
                tun: range.tun,
                key: Aes128Ctr::new(range.key.as_bytes()),
                mode,
                lan_ip_addr,
                try_send_to_lan_addr: range.try_send_to_lan_addr.unwrap_or(false),
            };
            ranges.push(range_finalize)
        }

        let config_finalize = ClientConfigFinalize {
            mtu: config.mtu.unwrap_or(1462),
            channel_limit: config.channel_limit.unwrap_or(100),
            api_addr: config
                .api_addr
                .unwrap_or_else(|| SocketAddr::from((Ipv4Addr::LOCALHOST, 3030))),
            tcp_heartbeat_interval: config
                .tcp_heartbeat_interval_secs
                .map(|sec| Duration::from_secs(ternary!(sec > 10, 10, sec)))
                .unwrap_or(Duration::from_secs(5)),
            udp_heartbeat_interval: config
                .udp_heartbeat_interval_secs
                .map(|sec| Duration::from_secs(ternary!(sec > 10, 10, sec)))
                .unwrap_or(Duration::from_secs(5)),
            reconnect_interval: Duration::from_secs(config.reconnect_interval_secs.unwrap_or(3)),
            udp_socket_recv_buffer_size: config.udp_socket_recv_buffer_size,
            udp_socket_send_buffer_size: config.udp_socket_send_buffer_size,
            tun_handler_thread_count: config.tun_handler_thread_count.unwrap_or(1),
            udp_handler_thread_count: config.udp_handler_thread_count.unwrap_or(1),
            network_ranges: ranges,
        };
        Ok(config_finalize)
    }
}

const INVALID_COMMAND: &str = "Invalid command";

macro_rules! block_on {
    ($expr: expr) => {{
        let rt = Runtime::new().context("Failed to build tokio runtime")?;
        let res = rt.block_on($expr);
        rt.shutdown_background();
        res
    }};
}

pub fn launch(server: &String, tun_ip: &String, tun_mask: &String, key: &String) -> Result<()> {
    let local_ip = crate::utils::get_local_ip()?;

    let config: ClientConfig = ClientConfig {
        mtu: Some(1462usize),
        channel_limit: Some(100usize),
        api_addr: None,
        tcp_heartbeat_interval_secs: Some(5u64),
        udp_heartbeat_interval_secs: Some(5u64),
        reconnect_interval_secs: Some(3u64),
        udp_socket_recv_buffer_size: Some(8196usize),
        udp_socket_send_buffer_size: Some(8196usize),
        tun_handler_thread_count: Some(1usize),
        udp_handler_thread_count: Some(1usize),
        network_ranges: vec![NetworkRange {
            server_addr: server.clone(),
            tun: TunIpAddr {
                ip: tun_ip.parse()?,
                netmask: tun_mask.parse()?,
            },
            key: key.clone(),
            mode: Some(String::from("UDP_AND_TCP")),
            lan_ip_addr: Some(IpAddr::V4(local_ip.parse()?)),
            try_send_to_lan_addr: Some(false),
        }],
    };
    let handle = Handle::current();
    std::thread::spawn(move || {
        handle.block_on(async move { client::start(ClientConfigFinalize::try_from(config)?).await })
    });
    Ok(())
}

fn load_config<T: de::DeserializeOwned>(path: &str) -> Result<T> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("Failed to read config from: {}", path))?;
    serde_json::from_reader(file).context("Failed to parse config")
}

fn logger_init() -> Result<()> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[Console] {d(%Y-%m-%d %H:%M:%S)} - {l} - {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::from_str(
                    &std::env::var("FUBUKI_LOG").unwrap_or_else(|_| String::from("INFO")),
                )?),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}
