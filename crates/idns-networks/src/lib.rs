#[macro_use]
extern crate log;

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

use tokio::runtime::Runtime;

use crate::common::cipher::Aes128Ctr;
use crate::common::net::proto::ProtocolMode;

mod common;
mod handlers;
mod node;
mod server;
mod tun;
mod tunnel;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Deserialize, Clone)]
pub struct Listener {
    listen_addr: SocketAddr,
    key: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    channel_limit: Option<usize>,
    tcp_heartbeat_interval_secs: Option<u64>,
    listeners: Vec<Listener>,
}

#[derive(Clone)]
pub struct ServerConfigFinalize {
    channel_limit: usize,
    tcp_heartbeat_interval: Duration,
    listeners: Vec<Listener>,
}

impl TryFrom<ServerConfig> for ServerConfigFinalize {
    type Error = anyhow::Error;

    fn try_from(config: ServerConfig) -> Result<Self> {
        let config_finalize = Self {
            channel_limit: config.channel_limit.unwrap_or(100),
            tcp_heartbeat_interval: config
                .tcp_heartbeat_interval_secs
                .map(|sec| Duration::from_secs(ternary!(sec > 10, 10, sec)))
                .unwrap_or(Duration::from_secs(5)),
            listeners: {
                for listener in &config.listeners {
                    if listener.listen_addr.ip().is_loopback() {
                        return Err(anyhow!("Listen address cannot be a loopback address"));
                    }
                }
                config.listeners
            },
        };

        Ok(config_finalize)
    }
}

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

#[derive(Clone)]
pub struct NetworkRangeFinalize {
    pub server_addr: String,
    pub tun: TunIpAddr,
    pub key: Aes128Ctr,
    pub mode: ProtocolMode,
    pub lan_ip_addr: Option<IpAddr>,
    pub try_send_to_lan_addr: bool,
}

pub const INVALID_COMMAND: &str = "Invalid command";

macro_rules! block_on {
    ($expr: expr) => {{
        let rt = Runtime::new().context("Failed to build tokio runtime")?;
        let res = rt.block_on($expr);
        rt.shutdown_background();
        res
    }};
}

pub fn cmd_launch() -> Result<()> {
    logger_init().unwrap();

    let config = ServerConfig {
        channel_limit: Some(100usize),
        tcp_heartbeat_interval_secs: Some(5u64),
        listeners: vec![Listener {
            listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 35093),
            key: String::from("a123"),
        }],
    };

    block_on!(async {
        server::start(ServerConfigFinalize::try_from(config)?).await;
        Ok(())
    })
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
