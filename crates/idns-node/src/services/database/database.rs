use crate::sqlite::{Queryer, Updater};
use bytes::Bytes;
use idns_eth_api::idns::system::{DatabaseQuery, DatabaseUpdate, StringMessage, U64Message};
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

    pub fn update_with_args(&self, sql: &String, args_json: &String) -> Result<usize> {
        let arc_conn = crate::get_connection()?;
        let res = Updater::update_with_args(&arc_conn, sql, args_json)?;
        Ok(res)
    }

    pub fn update(&self, sql: &String) -> Result<usize> {
        let arc_conn = crate::get_connection()?;
        let res = Updater::update(&arc_conn, sql)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl Handler for DatabaseServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
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
            } else if method_name == "update_with_args" {
                //
                let request = DatabaseUpdate::decode(Bytes::from(message))?;
                //
                return response(
                    self.update_with_args(&request.sql, &request.args_json)
                        .map(|r| U64Message { data: r as u64 }),
                );
            } else if method_name == "update" {
                //
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.update(&request.data)
                        .map(|r| U64Message { data: r as u64 }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
