#[macro_use]
extern crate lazy_static;

pub(crate) mod context;
pub mod executor;
pub mod scheduler;
pub mod task;
pub(crate) mod utils;

use idns_eth_api::{
    idns::system::{Command, CommandResponse},
    Error, Result,
};

lazy_static! {
    pub static ref SCHEDULER_DEFAULT: scheduler::Scheduler = scheduler::Scheduler::new();
}

//
pub async fn async_execute(request: Command) -> Result<CommandResponse> {
    //

    Err(Error::NotFoundService)
}
