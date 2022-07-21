use anyhow::{Error, Result};
use delay_timer::entity::{DelayTimer, DelayTimerBuilder};
use delay_timer::prelude::*;
use idns_eth_core::account::IdnsToken;
use rusqlite::Connection;
use std::sync::Arc;
use std::thread::{current, park, Thread};
//
pub struct Worker {
    delay_timer: DelayTimer,
    connection: Arc<Connection>,
    token: IdnsToken,
}

impl Drop for Worker {
    #[inline]
    fn drop(&mut self) {}
}

impl Worker {
    pub fn new(connection: Arc<Connection>, token: &IdnsToken) -> Self {
        let delay_timer = DelayTimerBuilder::default().build();

        Self {
            delay_timer,
            connection,
            token: token.clone(),
        }
    }

    pub fn start(&self) -> Result<()> {
        //
        self.delay_timer.add_task(
            TaskBuilder::default()
                .set_frequency_repeated_by_cron_str("0/6 * * * * ?")
                .set_maximum_running_time(15)
                .spawn_async_routine(|| async {
                    //
                    println!("ssssssss");
                })?,
        )?;
        Ok(())
    }

    pub fn close(self) -> Result<(), (Worker, Error)> {
        let _ = self
            .delay_timer
            .stop_delay_timer()
            .map_err(|err| (self, err));
        Ok(())
    }
}
