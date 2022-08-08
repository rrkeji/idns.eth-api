use crate::sqlite::Queryer;
use bytes::Bytes;
use idns_eth_api::idns::system::{DatabaseQuery, StringMessage};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;
pub struct DatabaseServiceImpl;

impl DatabaseServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatabaseServiceImpl {
    pub fn query(&self, sql: &String, size: usize) -> Result<String> {
        let arc_conn = crate::get_connection()?;
        let res = Queryer::query(&arc_conn, sql, size)?;
        Ok(res)
    }
}

impl Handler for DatabaseServiceImpl {
    fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.database" {
            if method_name == "query" {
                //
                let request = DatabaseQuery::decode(Bytes::from(message))?;
                //
                return response(
                    self.query(&request.sql, request.size as usize)
                        .map(|r| StringMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
