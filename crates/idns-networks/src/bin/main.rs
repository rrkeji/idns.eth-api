#[macro_use]
extern crate log;

use idns_eth_networks::cmd_launch;

fn main() {
    if let Err(e) = cmd_launch() {
        error!("Process error -> {:?}", e)
    };
}
