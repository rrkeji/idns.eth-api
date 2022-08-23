use bytes::Bytes;
use idns_eth_api::idns::identity::{IdentityEntity, ListIdentitiesResponse};
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
        idns_eth_core::identity::IdnsIdentity::query_identity_by_identity(&crate::get_password()?)
            .await?
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
            }
        }
        Err(Error::NotFoundService)
    }
}
