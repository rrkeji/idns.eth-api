use bytes::Bytes;
use idns_eth_api::idns::identity::{
    AttributeValueCreateRequest, AttributeValueEntity, ListAttributeValuesResponse,
};
use idns_eth_api::idns::system::{BoolMessage, StringMessage, StringPairMessage, U64Message};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;

pub struct AttributeValueServiceImpl {}

impl AttributeValueServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl AttributeValueServiceImpl {
    ///
    pub async fn attribute_value_list_by_source(
        &self,
        source_identity: &String,
    ) -> Result<Vec<AttributeValueEntity>> {
        let res = idns_eth_core::identity::IdnsAttributeValue::attribute_value_list_by_source(
            source_identity,
        )
        .await?;
        Ok(res)
    }
    ///
    pub async fn attribute_value_list_by_holder(
        &self,
        holder_identity: &String,
    ) -> Result<Vec<AttributeValueEntity>> {
        let res = idns_eth_core::identity::IdnsAttributeValue::attribute_value_list_by_holder(
            holder_identity,
        )
        .await?;
        Ok(res)
    }
    pub async fn query_one_attribute_value(&self, id: &String) -> Result<AttributeValueEntity> {
        let res =
            idns_eth_core::identity::IdnsAttributeValue::query_one_attribute_value(id).await?;
        Ok(res)
    }
    pub async fn create_attribute_value(
        &self,
        request: &AttributeValueCreateRequest,
    ) -> Result<bool> {
        let _ =
            idns_eth_core::identity::IdnsAttributeValue::create_attribute_value(request).await?;
        Ok(true)
    }
    pub async fn remove_attribute_value(&self, id: &String) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsAttributeValue::remove_attribute_value(id).await?;
        Ok(true)
    }
}

#[async_trait::async_trait]
impl Handler for AttributeValueServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.identity.attribute_value" {
            if method_name == "attribute_value_list_by_source" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.attribute_value_list_by_source(&request.data)
                        .await
                        .map(|r| ListAttributeValuesResponse {
                            attribute_values: r,
                        }),
                );
            } else if method_name == "attribute_value_list_by_holder" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.attribute_value_list_by_holder(&request.data)
                        .await
                        .map(|r| ListAttributeValuesResponse {
                            attribute_values: r,
                        }),
                );
            } else if method_name == "query_one_attribute_value" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(self.query_one_attribute_value(&request.data).await);
            } else if method_name == "remove_attribute_value" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.remove_attribute_value(&request.data)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "create_attribute_value" {
                let mut request = AttributeValueCreateRequest::decode(Bytes::from(message))?;
                request.holder_identity = crate::utils::get_identity()?.0;
                //
                return response(
                    self.create_attribute_value(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
