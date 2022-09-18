use anyhow::{anyhow, Context, Result};
use chacha20poly1305::{
    aead::{stream, NewAead},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, Rng, RngCore};
use serde_json::json;
use sp_core::sr25519;
use sp_core::{hexdisplay::HexDisplay, Pair};

use crate::kvstore::_kvstore_post_request;

#[derive(Debug, Clone)]
pub struct IdnsToken {
    pub public_key: String,
    pub application_key: String,
    pub token: String,
}

unsafe impl Send for IdnsToken {}

impl IdnsToken {
    /// 根据外部应用的token
    pub fn new_from_application_token(
        public_key: &String,
        application_key: &String,
        token: &String,
    ) -> IdnsToken {
        IdnsToken {
            public_key: public_key.clone(),
            application_key: application_key.clone(),
            token: token.clone(),
        }
    }
}

/// 通过账户的public_key登录
///
/// public_key phrase
///
/// 返回 Token
pub async fn login(application_key: &str, public_key: &str, phrase: &str) -> Result<IdnsToken> {
    //
    if let Ok((from, _)) = sr25519::Pair::from_phrase(phrase, None) {
        //
        let public_key_from_phrase = format!("{}", HexDisplay::from(&from.public().as_ref()));
        if public_key_from_phrase == public_key || public_key == "" {
            let body = {
                let mut rng = rand::thread_rng();
                let nonce = format!("{}", rng.next_u32());
                let signature = from.sign(&nonce.as_bytes());
                let signature_str = format!("{}", HexDisplay::from(&signature.as_ref()));
                &json!({
                    "public_key": public_key_from_phrase,
                    "application_key":application_key,
                    "signature":signature_str,
                    "nonce": nonce}
                )
                .to_string()
            };
            tracing::debug!("{}", body);
            //本地校验public key
            if let Ok(token) =
                _kvstore_post_request("/login", "application/json;charset=UTF-8", body, None)
                    .await
                    .with_context(|| format!("Fail login with public key {}!", public_key))
            {
                tracing::debug!("{}", token);
                let idns_token = IdnsToken::new_from_application_token(
                    &public_key_from_phrase.clone(),
                    &String::from(application_key),
                    &token,
                );

                // 空判断
                Ok(idns_token)
            } else {
                Err(anyhow!("Fail login, public_key {}!", public_key))
            }
        } else {
            Err(anyhow!("Fail login, public_key {}!", public_key))
        }
    } else {
        Err(anyhow!("Fail login, phrase {}!", phrase))
    }
}

fn argon2_config<'a>() -> argon2::Config<'a> {
    return argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };
}

pub fn encrypt_message(text: &String, password: &str) -> Result<(String, String, String, String)> {
    let argon2_config = argon2_config();

    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    let ciphertext = stream_encryptor
        .encrypt_last(text.as_bytes())
        .map_err(|err| anyhow!("Encrypting large file: {}", err))?;

    let mut account_id = String::new();
    if let Ok((pair, seed)) = sp_core::sr25519::Pair::from_phrase(text, None) {
        account_id = format!("{}", HexDisplay::from(&pair.public().as_ref()));
    }

    // 保存 salt account_id nonce ciphertext
    Ok((
        crate::utils::encode_b64(&salt),
        account_id,
        crate::utils::encode_b64(&nonce),
        crate::utils::encode_b64(&ciphertext),
    ))
}

pub fn decrypt_message(
    saltStr: &String,
    nonceStr: &String,
    plaintext: &String,
    password: &str,
) -> Result<String> {
    let salt = crate::utils::decode_b64(saltStr)?;
    let nonce = crate::utils::decode_b64(nonceStr)?;
    //读取并赋值salt nonce ciphertext
    let argon2_config = argon2_config();

    let key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce[..].as_ref().into());

    let plaintext = stream_decryptor
        .decrypt_last(&crate::utils::decode_b64(plaintext)?[..])
        .map_err(|err| anyhow!("Decrypting large file: {}", err))?;

    // 获取到plaintext
    Ok(String::from_utf8(plaintext)?)
}
