use anyhow::anyhow;
use bytes::Bytes;
pub use idns_eth_api::runnerc::account::{
    Account, DecryptMessageRequest, DecryptMessageResponse, EncryptMessageRequest,
    EncryptMessageResponse, GenerateAccountResponse, GetAccountByIdRequest, LoginRequest,
    LoginResponse,
};
use idns_eth_api::{response, Command, CommandResponse, EmptyMessage, Error, Handler, Result};
use prost::Message;
use tokio::runtime::Handle;

use crate::idns_core::account::{
    decrypt_message as decrypt_message_impl, encrypt_message as encrypt_message_impl,
    login as login_impl, Account as AccountImpl,
};

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

    pub fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        let application_key = request.application_key.clone();
        let public_key = request.public_key.clone();
        let phrase = request.phrase.clone();

        //
        let handle = Handle::current();
        let handle_std = std::thread::spawn(move || {
            handle.block_on(async move {
                if let Ok(token) = login_impl(
                    application_key.as_str(),
                    public_key.as_str(),
                    phrase.as_str(),
                )
                .await
                {
                    Ok(LoginResponse {
                        application_key,
                        public_key,
                        token: token.token.map_or(String::new(), |r| r),
                    })
                } else {
                    Err(anyhow!(""))
                }
            })
        });

        match handle_std.join() {
            Ok(res) => Ok(res?),
            Err(_err) => Err(Error::RuntimeVersion),
        }
    }

    pub fn encrypt_message(
        &self,
        request: EncryptMessageRequest,
    ) -> Result<EncryptMessageResponse> {
        //

        let plain = request.plain;
        let password = request.password;

        //使用password进行加密，并保存到相应的文件中
        let (salt, account_id, nonce, ciphertext) =
            encrypt_message_impl(&plain, password.clone().as_str())?;

        Ok(EncryptMessageResponse {
            salt,
            account_id,
            nonce,
            cipher: ciphertext,
        })
    }

    pub fn decrypt_message(
        &self,
        request: DecryptMessageRequest,
    ) -> Result<DecryptMessageResponse> {
        //

        let salt = request.salt;
        let nonce = request.nonce;
        let cipher = request.cipher;
        let password = request.password;

        if password == "" || nonce == "" || cipher == "" || salt == "" {
            Ok(DecryptMessageResponse {
                plain: String::new(),
            })
        } else {
            //使用password进行加密，并保存到相应的文件中
            let plain = decrypt_message_impl(&salt, &nonce, &cipher, password.clone().as_str())?;
            Ok(DecryptMessageResponse { plain })
        }
    }
}

impl Handler for AccountServiceImpl {
    fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.account" {
            if method_name == "generate_account" {
                //
                return response(self.generate_account());
            } else if method_name == "encrypt_message" {
                //
                return response(
                    self.encrypt_message(EncryptMessageRequest::decode(Bytes::from(message))?),
                );
            } else if method_name == "decrypt_message" {
                //
                return response(
                    self.decrypt_message(DecryptMessageRequest::decode(Bytes::from(message))?),
                );
            } else if method_name == "login" {
                //
                return response(self.login(LoginRequest::decode(Bytes::from(message))?));
            }
        }
        Err(Error::NotFoundService)
    }
}
