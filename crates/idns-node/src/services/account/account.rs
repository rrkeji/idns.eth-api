use idns_eth_api::idns::account::GenerateAccountResponse;
use idns_eth_api::idns::identity::IdentityEntity;
use idns_eth_api::idns::system::StringMessage;
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use idns_eth_core::account::Account as AccountImpl;

pub struct AccountServiceImpl;

impl AccountServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl AccountServiceImpl {
    pub fn generate_account(&self) -> Result<GenerateAccountResponse> {
        let res = AccountImpl::generate_account()?;

        Ok(GenerateAccountResponse {
            account_id: res.0.clone(),
            secret_phrase: res.1.clone(),
            secret_seed: res.0.clone(),
            public_key: res.0.clone(),
        })
    }
    pub fn get_user_public_key(&self) -> Result<String> {
        let res = idns_eth_core::get_user_public_key()?;
        Ok(res)
    }
    pub fn get_account_public_key(&self) -> Result<String> {
        let res = idns_eth_core::get_account_public_key()?;
        Ok(res)
    }

    pub async fn get_account_identity(&self) -> Result<IdentityEntity> {
        let public_key = idns_eth_core::get_account_public_key()?;

        get_account_identity_by_public_key(&public_key).await
    }
}

async fn get_account_identity_by_public_key(account_public_key: &String) -> Result<IdentityEntity> {
    let res =
        idns_eth_core::identity::IdnsIdentity::query_identity_by_public_key(account_public_key)
            .await?;
    Ok(res)
}

#[async_trait::async_trait]
impl Handler for AccountServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;

        if service_name == "idns.system.account" {
            if method_name == "generate_account" {
                //
                return response(self.generate_account());
            } else if method_name == "get_user_public_key" {
                //
                return response(
                    self.get_user_public_key()
                        .map(|r| StringMessage { data: r }),
                );
            } else if method_name == "get_account_public_key" {
                //
                return response(
                    self.get_account_public_key()
                        .map(|r| StringMessage { data: r }),
                );
            } else if method_name == "get_account_identity" {
                //
                return response(self.get_account_identity().await);
            }
        }
        Err(Error::NotFoundService)
    }
}
