#[macro_use]
extern crate log;

pub(crate) mod utils;
pub mod vpnc;

use anyhow::Result;

pub fn launch() -> Result<()> {
    // let _ = crate::vpnc::launch(server, tun_ip, tun_mask, key)?;
    Ok(())
}
