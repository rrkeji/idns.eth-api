use idns_eth_devices::*;
use std::{thread, time};

fn main() {
    start_server(false);

    //510536204 rqmxbf
    ui::start_with_peer(String::from("510536204"), String::from("rqmxbf"));

    // let ten_millis = time::Duration::from_millis(1000000);
    // thread::sleep(ten_millis);
}
