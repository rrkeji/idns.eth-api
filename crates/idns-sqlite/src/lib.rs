#[macro_use]
extern crate lazy_static;

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/idns.sqlite.types.rs"));
}

use delay_timer::entity::{DelayTimer, DelayTimerBuilder};
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = HashMap::new();
    static ref DELAY_TIMER_REF: DelayTimer = DelayTimerBuilder::default().build();
}

mod connection;
pub(crate) mod sync;
pub(crate) mod utils;
// pub(crate) use idns_eth_ipfs_client as ipfs_api;
pub use ipfs_api_backend_hyper as ipfs_api;

pub use connection::*;
