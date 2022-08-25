#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub(crate) mod utils;
pub mod vpnc;

use anyhow::{anyhow, Context, Result};
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;

lazy_static! {
    pub(crate) static ref VPNS_HANDLER: RwLock<Option<JoinHandle<std::result::Result<(), anyhow::Error>>>> =
        RwLock::new(None);
}

pub(crate) fn set_vpns_handler(
    handler: JoinHandle<std::result::Result<(), anyhow::Error>>,
) -> Result<()> {
    let mut w = crate::VPNS_HANDLER
        .write()
        .map_err(|err| anyhow!("获取锁失败"))?;
    *w = Some(handler);
    Ok(())
}

pub(crate) fn clean_vpns_handler() -> Result<()> {
    let mut w = VPNS_HANDLER.write().map_err(|err| anyhow!("获取锁失败"))?;
    match w.take() {
        Some(handler) => handler.abort(),
        _ => {}
    }
    Ok(())
}

pub fn launch(tun_ip: &String) -> Result<()> {
    // let _ = crate::vpnc::launch(server, tun_ip, tun_mask, key)?;
    let _ = crate::vpnc::launch(
        &String::from("49.232.102.140:35093"),
        tun_ip,
        &String::from("255.255.255.0"),
        &String::from("a123"),
    )?;
    Ok(())
}
