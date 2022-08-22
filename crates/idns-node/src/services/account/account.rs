use idns_eth_api::idns::account::GenerateAccountResponse;
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
            }
        }
        Err(Error::NotFoundService)
    }
}
