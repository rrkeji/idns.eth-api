use anyhow::anyhow;
use bytes::Bytes;
use idns_eth_api::idns::account::{
    DecryptMessageRequest, DecryptMessageResponse, EncryptMessageRequest, EncryptMessageResponse,
    LoginRequest, LoginResponse,
};
use idns_eth_api::idns::system::{BoolMessage, StringMessage};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Handle;

#[derive(Deserialize, Serialize)]
struct AccountJson {
    pub salt: String,
    pub public_key: String,
    pub nonce: String,
    pub cipher_text: String,
}

use idns_eth_core::account::{
    decrypt_message as decrypt_message_impl, encrypt_message as encrypt_message_impl,
    login as login_impl, IdnsToken,
};
pub struct AuthServiceImpl;

impl AuthServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl AuthServiceImpl {
    /// 是否在线
    pub async fn is_online(&self) -> Result<bool> {
        let token = crate::get_token()?;

        if let Ok(_) = idns_eth_core::kvstore::KVStore::get_value(&token).await {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn unimport(&self) -> Result<bool> {
        //TODO 检查是否有数据未保存
        idns_eth_core::utils::files::file_delete("", "account.json")?;

        Ok(true)
    }

    pub async fn is_login(&self) -> Result<bool> {
        //
        self.is_online().await
    }

    /// 判断是否已经导入账号
    pub fn is_imported(&self) -> Result<bool> {
        //查看文件是否存在
        let exist = idns_eth_core::utils::files::file_exists("", "account.json")?;
        if !exist {
            return Ok(false);
        }

        //判断是否有account.json文件
        let json_str = idns_eth_core::utils::files::read_string_from_file("", "account.json")?;
        let account: AccountJson =
            serde_json::from_str(json_str.as_str()).map_err(|e| anyhow!("{}", e))?;
        //
        Ok(account.public_key.as_str() != "")
    }
    //
    pub async fn login_by_password(&self, password: &String) -> Result<IdnsToken> {
        //读取文件中的内容， 进行尝试解密
        let json_str = idns_eth_core::utils::files::read_string_from_file("", "account.json")?;
        //
        let account: AccountJson =
            serde_json::from_str(json_str.as_str()).map_err(|e| anyhow!("{}", e))?;
        let plain = decrypt_message_impl(
            &account.salt,
            &account.nonce,
            &account.cipher_text,
            password.clone().as_str(),
        )?;

        self.import_and_login(&plain, password, false).await
    }

    pub async fn reset_password(
        &self,
        old_password: &String,
        new_password: &String,
    ) -> Result<bool> {
        //比较原密码是否一致
        let password = crate::get_password()?;
        if password.as_str() != old_password.as_str() {
            //
            return Ok(false);
        }
        //
        let json_str = idns_eth_core::utils::files::read_string_from_file("", "account.json")?;
        //
        let account: AccountJson =
            serde_json::from_str(json_str.as_str()).map_err(|e| anyhow!("{}", e))?;
        let plain = decrypt_message_impl(
            &account.salt,
            &account.nonce,
            &account.cipher_text,
            old_password.clone().as_str(),
        )?;
        //重新保存文件
        self.import_and_login(&plain, new_password, true).await?;

        Ok(false)
    }

    pub async fn import_and_login(
        &self,
        phrase: &String,
        password: &String,
        update_file: bool,
    ) -> Result<IdnsToken> {
        //
        let application_key = crate::get_Application_key()?;
        let token = self
            .login(application_key.clone(), String::new(), phrase.clone())
            .await?;
        //
        //加密
        if let Ok((salt, account_id, nonce, ciphertext)) =
            encrypt_message_impl(&phrase, password.as_str())
        {
            let file_content = json!(AccountJson {
                salt,
                public_key: account_id,
                nonce,
                cipher_text: ciphertext
            })
            .to_string();

            if update_file {
                //保存文件成功之后在保存密码到内存中
                if let Ok(_) = idns_eth_core::utils::files::write_to_file(
                    "",
                    "account.json",
                    &file_content.as_bytes().to_vec(),
                ) {
                    //保存密码
                    {
                        let mut w = crate::PASSWORD.write().unwrap();
                        *w = Some(password.clone());
                    }
                    return Ok(token.clone());
                }
            } else {
                //保存密码
                {
                    let mut w = crate::PASSWORD.write().unwrap();
                    *w = Some(password.clone());
                }
                return Ok(token);
            }
        }
        Err(anyhow!(""))?
    }

    pub async fn login_by_identity(&self, identity: &String) -> Result<IdnsToken> {
        //获取到identity的信息
        let identity_entity = crate::services::IdentityServiceImpl::new()
            .query_identity_by_identity(identity)
            .await?;
        //解密出秘钥
        let password = _get_hash()?;

        tracing::debug!("login_by_identity:{}-{}", identity, password);

        let salt = identity_entity.sr25519_salt.clone();
        let cipher = identity_entity.sr25519_ciphertext.clone();
        let nonce = identity_entity.sr25519_nonce.clone();

        tracing::debug!("cipher:{} salt:{} nonce:{}", cipher, salt, nonce);
        let identity_phrase =
            decrypt_message_impl(&salt, &nonce, &cipher, password.clone().as_str())?;

        tracing::debug!("login_by_identity:{}-{}", identity, identity_phrase);

        self.import_and_login(&identity_phrase, &crate::get_password()?, true)
            .await
    }

    pub async fn user_import_and_login(
        &self,
        phrase: &String,
        password: &String,
    ) -> Result<IdnsToken> {
        //
        let application_key = crate::get_Application_key()?;
        let token = self
            .login(application_key.clone(), String::new(), phrase.clone())
            .await?;
        //
        //加密
        if let Ok((salt, account_id, nonce, ciphertext)) =
            encrypt_message_impl(&phrase, password.as_str())
        {
            let file_content = json!(AccountJson {
                salt,
                public_key: account_id,
                nonce,
                cipher_text: ciphertext
            })
            .to_string();

            //保存文件成功之后在保存密码到内存中
            if let Ok(_) = idns_eth_core::utils::files::write_to_file(
                "",
                "user.json",
                &file_content.as_bytes().to_vec(),
            ) {
                //保存密码
                {
                    let mut w = crate::PASSWORD.write().unwrap();
                    *w = Some(password.clone());
                }

                return Ok(token.clone());
            }
        }
        Err(anyhow!(""))?
    }
    ///登出
    pub fn logout(&self) -> Result<bool> {
        {
            let mut w = crate::TOKEN.write().unwrap();
            *w = None;
        }
        {
            let mut w = crate::PASSWORD.write().unwrap();
            *w = None;
        }
        //
        Ok(true)
    }

    pub async fn login(
        &self,
        application_key: String,
        public_key: String,
        phrase: String,
    ) -> Result<IdnsToken> {
        if let Ok(token) = login_impl(
            application_key.as_str(),
            public_key.as_str(),
            phrase.as_str(),
        )
        .await
        {
            let idns_token = IdnsToken {
                application_key: token.application_key,
                public_key: token.public_key,
                token: token.token,
            };
            let _ = crate::utils::set_token(&idns_token)?;
            Ok(idns_token)
        } else {
            Err(anyhow!(""))?
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

#[async_trait::async_trait]
impl Handler for AuthServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.auth" {
            if method_name == "encrypt_message" {
                //
                return response(
                    self.encrypt_message(EncryptMessageRequest::decode(Bytes::from(message))?),
                );
            } else if method_name == "decrypt_message" {
                //
                return response(
                    self.decrypt_message(DecryptMessageRequest::decode(Bytes::from(message))?),
                );
            } else if method_name == "is_online" {
                //
                let res = self.is_online().await;
                return response(res.map(|r| BoolMessage { data: r }));
            } else if method_name == "unimport" {
                //
                let res = self.unimport().await;
                return response(res.map(|r| BoolMessage { data: r }));
            } else if method_name == "is_login" {
                //
                let res = self.is_login().await;
                return response(res.map(|r| BoolMessage { data: r }));
            } else if method_name == "is_imported" {
                //
                return response(self.is_imported().map(|r| BoolMessage { data: r }));
            } else if method_name == "logout" {
                //
                return response(self.logout().map(|r| BoolMessage { data: r }));
            } else if method_name == "login_by_password" {
                //
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                let res = self.login_by_password(&request.data).await;
                return response(res.map(|r| LoginResponse {
                    application_key: r.application_key.clone(),
                    public_key: r.public_key.clone(),
                    token: r.token.clone(),
                }));
            } else if method_name == "import_and_login" {
                //
                let request = LoginRequest::decode(Bytes::from(message))?;

                return response(
                    self.import_and_login(&request.phrase, &request.password, true)
                        .await
                        .map(|r| LoginResponse {
                            application_key: r.application_key.clone(),
                            public_key: r.public_key.clone(),
                            token: r.token.clone(),
                        }),
                );
            } else if method_name == "login_by_identity" {
                //
                let request = StringMessage::decode(Bytes::from(message))?;

                return response(self.login_by_identity(&request.data).await.map(|r| {
                    LoginResponse {
                        application_key: r.application_key.clone(),
                        public_key: r.public_key.clone(),
                        token: r.token.clone(),
                    }
                }));
            } else if method_name == "user_import_and_login" {
                //
                let request = LoginRequest::decode(Bytes::from(message))?;

                return response(
                    self.user_import_and_login(&request.phrase, &request.password)
                        .await
                        .map(|r| LoginResponse {
                            application_key: r.application_key.clone(),
                            public_key: r.public_key.clone(),
                            token: r.token.clone(),
                        }),
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
