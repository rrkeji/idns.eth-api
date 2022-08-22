use crate::get_connection;
use bytes::Bytes;
use idns_eth_api::idns::system::{BoolMessage, StringMessage, U64Message};
use idns_eth_api::idns::wasmer::{ListTasksResponse, TaskEntity};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use idns_eth_wasmer::task::TaskServiceImpl;
use prost::Message;
pub struct SdkTaskServiceImpl {
    service_impl: TaskServiceImpl,
}

impl SdkTaskServiceImpl {
    pub fn new() -> Self {
        Self {
            service_impl: TaskServiceImpl::new(get_connection().unwrap()),
        }
    }
}

#[async_trait::async_trait]
impl Handler for SdkTaskServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.wasmer.task" {
            if method_name == "list_tasks" {
                //
                let device_uuid = StringMessage::decode(Bytes::from(message))?;

                //
                return response(
                    self.service_impl
                        .list_tasks(&device_uuid.data)
                        .map(|r| ListTasksResponse { tasks: r }),
                );
            } else if method_name == "list_deleted_tasks" {
                //
                let device_uuid = StringMessage::decode(Bytes::from(message))?;

                return response(
                    self.service_impl
                        .list_deleted_tasks(&device_uuid.data)
                        .map(|r| ListTasksResponse { tasks: r }),
                );
            } else if method_name == "create_task" {
                //
                let request = TaskEntity::decode(Bytes::from(message))?;

                return response(
                    self.service_impl
                        .create_task(&request)
                        .map(|r| U64Message { data: r }),
                );
            } else if method_name == "delete_task" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.service_impl
                        .delete_task(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "recovery_task" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.service_impl
                        .recovery_task(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_task" {
                //
                let request = TaskEntity::decode(Bytes::from(message))?;

                return response(
                    self.service_impl
                        .update_task(&request)
                        .map(|r| U64Message { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
