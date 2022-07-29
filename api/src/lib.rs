mod command;
mod error;

pub use command::*;
pub use error::*;
//
pub mod runnerc {
	//
	pub mod networks {
		include!(concat!(env!("OUT_DIR"), "/runnerc.networks.rs"));
	}
	//
	pub mod account {
		include!(concat!(env!("OUT_DIR"), "/runnerc.account.rs"));
	}
	//
	pub mod service {
		include!(concat!(env!("OUT_DIR"), "/runnerc.service.rs"));
	}
	//
	pub mod system {
		include!(concat!(env!("OUT_DIR"), "/runnerc.system.rs"));
	}
}

//
pub mod udi {
	//
	pub mod worker {
		include!(concat!(env!("OUT_DIR"), "/udi.worker.rs"));
	}
}
