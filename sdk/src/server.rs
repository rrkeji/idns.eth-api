use crate::idns_core::Server;
use anyhow::Result;

pub fn server_main() -> Result<()> {
    Server::start()
}
