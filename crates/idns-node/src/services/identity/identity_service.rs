use bytes::Bytes;
use idns_eth_api::idns::identity::{
    IdentityCreateRequest, IdentityEntity, IdentityUpdateRequest, ListIdentitiesResponse,
};
use idns_eth_api::idns::system::{BoolMessage, BytesMessage, StringMessage, U64Message};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;

pub struct IdentityServiceImpl {}

impl IdentityServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl IdentityServiceImpl {
    ///
    pub async fn list_identities(&self) -> Result<Vec<IdentityEntity>> {
        let res =
            idns_eth_core::identity::IdnsIdentity::identities(&crate::get_password()?).await?;
        Ok(res)
    }

    pub async fn query_identity_by_identity(&self, identity: &String) -> Result<IdentityEntity> {
        let res = idns_eth_core::identity::IdnsIdentity::query_identity_by_identity(
            &crate::get_password()?,
            identity,
        )
        .await?;
        Ok(res)
    }

    pub async fn query_identity_by_public_key(
        &self,
        public_key: &String,
    ) -> Result<IdentityEntity> {
        let res =
            idns_eth_core::identity::IdnsIdentity::query_identity_by_public_key(public_key).await?;
        Ok(res)
    }

    pub async fn create_identity(
        &self,
        request: &IdentityCreateRequest,
        hash: &String,
    ) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsIdentity::create_identity(
            &crate::get_password()?,
            request,
            hash,
        )
        .await?;
        Ok(true)
    }
    pub async fn remove_identity(&self, identity: &String) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsIdentity::remove_identity(
            &crate::get_password()?,
            identity,
        )
        .await?;
        Ok(true)
    }
    pub async fn update_identity(&self, request: &IdentityUpdateRequest) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsIdentity::update_identity(
            &crate::get_password()?,
            request,
        )
        .await?;
        Ok(true)
    }
}

#[async_trait::async_trait]
impl Handler for IdentityServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.identity.identity" {
            if method_name == "list_identities" {
                //
                return response(
                    self.list_identities()
                        .await
                        .map(|r| ListIdentitiesResponse { identities: r }),
                );
            } else if method_name == "query_identity_by_identity" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(self.query_identity_by_identity(&request.data).await);
            } else if method_name == "remove_identity" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.remove_identity(&request.data)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "create_identity" {
                let request = IdentityCreateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.create_identity(&request, &_get_hash()?)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_identity" {
                let request = IdentityUpdateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.update_identity(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}

fn _get_hash() -> Result<String> {
    //
    let phrase = idns_eth_core::get_user_phrase(&crate::get_password()?)?;
    let digest = md5::compute(phrase.as_str());
    Ok(format!("{:x}", digest))
}
