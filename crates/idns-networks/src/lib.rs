#[macro_use]
extern crate log;

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
extern crate sys_info;

pub mod device;
// pub mod rpc;
pub(crate) mod utils;
pub mod vpnc;
