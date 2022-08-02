mod command;
mod error;

pub use command::*;
pub use error::*;
//
pub mod idns {
    //
    pub mod networks {
        include!(concat!(env!("OUT_DIR"), "/idns.networks.rs"));
    }
    //
    pub mod account {
        include!(concat!(env!("OUT_DIR"), "/idns.account.rs"));
    }
    //
    pub mod service {
        include!(concat!(env!("OUT_DIR"), "/idns.service.rs"));
    }
    //
    pub mod storage {
        include!(concat!(env!("OUT_DIR"), "/idns.storage.rs"));
    }
    //
    pub mod system {
        include!(concat!(env!("OUT_DIR"), "/idns.system.rs"));
    }
    //
    pub mod worker {
        include!(concat!(env!("OUT_DIR"), "/idns.worker.rs"));
    }
}
