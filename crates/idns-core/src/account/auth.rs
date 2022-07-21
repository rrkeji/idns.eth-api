use anyhow::{anyhow, Context, Result};
use sp_core::sr25519;
use sp_core::Pair;

use crate::kvstore::_kvstore_get_request;

#[derive(Debug, Clone)]
pub struct IdnsToken {
    pub public_key: Option<String>,
    pub application_key: Option<String>,
    pub token: Option<String>,
}

impl IdnsToken {
    ///
    pub fn new() -> Self {
        Self {
            public_key: None,
            application_key: None,
            token: None,
        }
    }
    /// 根据public key构建, 只读使用的token
    pub fn new_from_public_key(public_key: &String) -> IdnsToken {
        IdnsToken {
            public_key: Some(public_key.clone()),
            application_key: None,
            token: None,
        }
    }

    /// 根据内部的token, 不需要public key, token中包含
    pub fn new_from_token(token: &String) -> IdnsToken {
        IdnsToken {
            public_key: None,
            application_key: None,
            token: Some(token.clone()),
        }
    }
    /// 根据外部应用的token
    pub fn new_from_application_token(
        public_key: &String,
        application_key: &String,
        token: &String,
    ) -> IdnsToken {
        IdnsToken {
            public_key: Some(public_key.clone()),
            application_key: Some(application_key.clone()),
            token: Some(token.clone()),
        }
    }
}

/// 通过账户的public_key登录
///
/// public_key phrase
///
/// 返回 Token
pub async fn login(public_key: &str, phrase: &str) -> Result<IdnsToken> {
    //
    if let Ok((from, _)) = sr25519::Pair::from_phrase(phrase, None) {
        //
        let public_key_from_phrase =
            super::format_public_key::<sp_core::sr25519::Pair>(from.public());

        if public_key_from_phrase == public_key {
            //本地校验public key
            let token = _kvstore_get_request(format!("/login/pk/{}", public_key).as_str(), None)
                .await
                .with_context(|| format!("Fail login with public key {}!", public_key))?;
            // 空判断
            Ok(IdnsToken::new_from_token(&token))
        } else {
            Err(anyhow!("Fail login, public_key {}!", public_key))
        }
    } else {
        Err(anyhow!("Fail login, phrase {}!", phrase))
    }
}
