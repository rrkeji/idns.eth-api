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
    pub mod identity {
        include!(concat!(env!("OUT_DIR"), "/idns.identity.rs"));
    }
    //
    pub mod wasmer {
        include!(concat!(env!("OUT_DIR"), "/idns.wasmer.rs"));
    }
    //
    pub mod storage {
        include!(concat!(env!("OUT_DIR"), "/idns.storage.rs"));
    }
    //
    pub mod system {
        include!(concat!(env!("OUT_DIR"), "/idns.system.rs"));
    }
}
