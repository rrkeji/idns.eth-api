#[macro_use]
extern crate log;

use idns_eth_networks::{device::get_device_info, vpnc::cmd_launch};

fn main() {
    println!("{:?}", get_device_info().unwrap());
    // if let Err(e) = cmd_launch() {
    //     error!("Process error -> {:?}", e)
    // };
}
