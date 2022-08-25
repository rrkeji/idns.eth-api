use bytes::Bytes;
use idns_eth_api::idns::identity::{
    ListMetaCredentialsResponse, MetaCredentialCreateRequest, MetaCredentialEntity,
    MetaCredentialUpdateRequest,
};
use idns_eth_api::idns::system::{BoolMessage, StringMessage, StringPairMessage, U64Message};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;

pub struct MetaCredentialServiceImpl {}

impl MetaCredentialServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl MetaCredentialServiceImpl {
    ///
    pub async fn meta_credential_list(
        &self,
        source_identity: &String,
    ) -> Result<Vec<MetaCredentialEntity>> {
        let res =
            idns_eth_core::identity::IdnsMetaCredential::meta_credential_list(source_identity)
                .await?;
        Ok(res)
    }

    pub async fn query_one_meta_credential(
        &self,
        source_identity: &String,
        name: &String,
    ) -> Result<MetaCredentialEntity> {
        let res = idns_eth_core::identity::IdnsMetaCredential::query_one_meta_credential(
            source_identity,
            name,
        )
        .await?;
        Ok(res)
    }
    pub async fn create_meta_credential(
        &self,
        request: &MetaCredentialCreateRequest,
    ) -> Result<bool> {
        let _ =
            idns_eth_core::identity::IdnsMetaCredential::create_meta_credential(request).await?;
        Ok(true)
    }
    pub async fn remove_meta_credential(
        &self,
        source_identity: &String,
        name: &String,
    ) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsMetaCredential::remove_meta_credential(
            source_identity,
            name,
        )
        .await?;
        Ok(true)
    }
    pub async fn update_meta_credential(
        &self,
        request: &MetaCredentialUpdateRequest,
    ) -> Result<bool> {
        let _ =
            idns_eth_core::identity::IdnsMetaCredential::update_meta_credential(request).await?;
        Ok(true)
    }
}

#[async_trait::async_trait]
impl Handler for MetaCredentialServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.identity.meta_credential" {
            if method_name == "meta_credential_list" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(self.meta_credential_list(&request.data).await.map(|r| {
                    ListMetaCredentialsResponse {
                        meta_credentials: r,
                    }
                }));
            } else if method_name == "query_one_meta_credential" {
                let request = StringPairMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.query_one_meta_credential(&request.first, &request.second)
                        .await,
                );
            } else if method_name == "remove_meta_credential" {
                let request = StringPairMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.remove_meta_credential(&request.first, &request.second)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "create_meta_credential" {
                let request = MetaCredentialCreateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.create_meta_credential(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_meta_credential" {
                let request = MetaCredentialUpdateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.update_meta_credential(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
