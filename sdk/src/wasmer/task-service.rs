use idns_eth_wasmer::task::TaskServiceImpl;
use crate::get_token;
use idns_eth_api::idns::wasmer::{TaskEntity,ListTasksResponse};

pub struct SdkTaskServiceImpl{
    impl:TaskServiceImpl,
}

impl SdkTaskServiceImpl{
    pub fn new()->Self{
        Self{
            impl:TaskServiceImpl::new(get_token())
        }
    }
}

impl Handler for SdkTaskServiceImpl {
    fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.task" {
            if method_name == "list_tasks" {
                 //
                 let device_uuid = StringMessage::decode(Bytes::from(message))?;
               
                //
                return response(
                    self.impl.list_tasks(&device_uuid.data)
                        .map(|r| ListTasksResponse { tasks: r }),
                );
            } else if method_name == "list_deleted_tasks" {
                 //
                 let device_uuid = StringMessage::decode(Bytes::from(message))?;
              
                return response(
                    self.impl.list_deleted_devices(&device_uuid.data)
                        .map(|r| ListDevicesResponse { devices: r }),
                );
            } else if method_name == "create_task" {
                //
                let request = TaskEntity::decode(Bytes::from(message))?;

                return response(self.impl.create_task(&request).map(|r| U64Message { data: r }));
            } else if method_name == "delete_task" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.impl.delete_task(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "recovery_task" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.impl.recovery_task(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_task" {
                //
                let request = TaskEntity::decode(Bytes::from(message))?;

                return response(self.impl.update_task(&request).map(|r| U64Message { data: r }));
            }
        }
        Err(Error::NotFoundService)
    }
}
