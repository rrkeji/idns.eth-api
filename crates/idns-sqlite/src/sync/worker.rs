use anyhow::Result;
use futures::executor::block_on;
use idns_eth_core::account::IdnsToken;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

//
pub struct Worker {
    sync_handle: JoinHandle<()>,
    token: IdnsToken,
}

unsafe impl Send for Worker {}
unsafe impl Sync for Worker {}

impl Drop for Worker {
    #[inline]
    fn drop(&mut self) {}
}

impl Worker {
    pub fn new(path: &String, token: &IdnsToken) -> Self {
        let token_clone: IdnsToken = token.clone();
        let str = path.clone();

        let sync_handle = thread::spawn(move || {
            block_on(async {
                loop {
                    let path2 = Path::new(str.as_str());
                    let token_inner: IdnsToken = token_clone.clone();
                    // let conn_inner = conn.clone();
                    if let Err(err) = crate::sync::DataBaseSync::data_sync(path2, token_inner).await
                    {
                        println!("err:{}", err);
                    }
                    thread::sleep(Duration::from_millis(1000));
                }
            })
        });
        Self {
            sync_handle,
            token: token.clone(),
        }
    }

    pub fn start(&self) -> Result<()> {
        Ok(())
    }

    // pub fn close(self) -> Result<(), (Worker, Error)> {
    //     let _ = self
    //         .delay_timer
    //         .stop_delay_timer()
    //         .map_err(|err| (self, err));
    //     Ok(())
    // }
}