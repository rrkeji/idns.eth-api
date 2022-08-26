use bytes::Bytes;
use idns_eth_api::idns::identity::{
    AttributeCreateRequest, AttributeEntity, AttributeUpdateRequest, ListAttributesResponse,
};
use idns_eth_api::idns::system::{BoolMessage, StringMessage, StringPairMessage, U64Message};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;

pub struct AttributeServiceImpl {}

impl AttributeServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl AttributeServiceImpl {
    ///
    pub async fn attribute_list(&self, source_identity: &String) -> Result<Vec<AttributeEntity>> {
        let res = idns_eth_core::identity::IdnsAttribute::attribute_list(source_identity).await?;
        Ok(res)
    }

    pub async fn query_one_attribute(&self, id: &String) -> Result<AttributeEntity> {
        let res = idns_eth_core::identity::IdnsAttribute::query_one_attribute(id).await?;
        Ok(res)
    }
    pub async fn create_attribute(&self, request: &AttributeCreateRequest) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsAttribute::create_attribute(request).await?;
        Ok(true)
    }
    pub async fn remove_attribute(&self, id: &String) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsAttribute::remove_attribute(id).await?;
        Ok(true)
    }
    pub async fn update_attribute(&self, request: &AttributeUpdateRequest) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsAttribute::update_attribute(request).await?;
        Ok(true)
    }
}

#[async_trait::async_trait]
impl Handler for AttributeServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.identity.attribute" {
            if method_name == "attribute_list" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.attribute_list(&request.data)
                        .await
                        .map(|r| ListAttributesResponse { attributes: r }),
                );
            } else if method_name == "query_one_attribute" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(self.query_one_attribute(&request.data).await);
            } else if method_name == "remove_attribute" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.remove_attribute(&request.data)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "create_attribute" {
                let request = AttributeCreateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.create_attribute(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_attribute" {
                let request = AttributeUpdateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.update_attribute(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
