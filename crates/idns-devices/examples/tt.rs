use idns_eth_devices::*;
use std::{thread, time};

fn main() {
    start_server(false);

    //510536204 rqmxbf
    ui::start_with_peer(String::from("127.0.0.1"), String::from("ikvdwn"));

    // let ten_millis = time::Duration::from_millis(1000000);
    // thread::sleep(ten_millis);
}
