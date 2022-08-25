#[macro_use]
extern crate log;

use idns_eth_networks::vpnc::launch;
use std::{thread, time};

#[tokio::main]
async fn main() {
    if let Err(e) = launch(
        &String::from("49.232.102.140:35093"),
        &String::from("10.0.0.4"),
        &String::from("255.255.255.0"),
        &String::from("a123"),
    ) {
        error!("Process error -> {:?}", e)
    };

    // if let Err(e) = launch(
    //     &String::from("49.232.102.140:35093"),
    //     &String::from("10.0.0.4"),
    //     &String::from("255.255.255.0"),
    //     &String::from("a123"),
    // ) {
    //     error!("Process error -> {:?}", e)
    // };
    let ten_millis = time::Duration::from_millis(1000000);
    thread::sleep(ten_millis);
}
