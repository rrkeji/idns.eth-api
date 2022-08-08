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
mod query;
pub mod sync;
mod update;
pub mod utils;
// pub(crate) use idns_eth_ipfs_client as ipfs_api;
pub use connection::*;
pub use ipfs_api_backend_hyper as ipfs_api;
pub use utils::{ipfs_add_content, ipfs_get_content};

pub use crate::{query::Queryer, update::Updater};
