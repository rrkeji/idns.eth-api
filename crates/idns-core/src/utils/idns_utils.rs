use anyhow::{anyhow, Context, Result};
use chacha20poly1305::{
    aead::{stream, NewAead},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, Rng, RngCore};
use serde_json::json;
use sp_core::sr25519;
use sp_core::{hexdisplay::HexDisplay, Pair};
use std::path::PathBuf;
/// 获取主目录
///
/// # 示例
///
/// ```
/// let storage_path = idns_home_path()?.join("sqlite");
/// std::fs::create_dir_all(storage_path.as_path()).unwrap();
/// ```
///
pub fn idns_home_path() -> Result<PathBuf> {
    let home_path = std::env::var_os("IDNS_ETH_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            home::home_dir().map(|tilde| {
                let mut path = PathBuf::from(tilde);
                path.push(".idns_eth");
                path
            })
        });
    //
    // let home_path = Some(PathBuf::from("/Users/suhs/jinisu/idns.eth-api/target"));
    //
    if let Some(home_path) = home_path {
        //
        std::fs::create_dir_all(home_path.as_path())?;
        Ok(home_path)
    } else {
        Err(anyhow!("没有设置IDNS_ETH_PATH路径"))
    }
}
//获取 phrase
pub fn get_signature_nonce(phrase: &String) -> Result<(String, String, String)> {
    //
    if let Ok((from, _)) = sr25519::Pair::from_phrase(phrase.as_str(), None) {
        //
        let public_key = format!("{}", HexDisplay::from(&from.public().as_ref()));

        let mut rng = rand::thread_rng();
        let nonce = format!("{}", rng.next_u32());
        let signature = from.sign(&nonce.as_bytes());
        let signature_str = format!("{}", HexDisplay::from(&signature.as_ref()));
        Ok((public_key, signature_str, nonce))
    } else {
        Err(anyhow!("助记词不合法!"))
    }
}

pub(crate) fn get_user_phrase(password: &String) -> Result<String> {
    //
    Ok(String::from(crate::account::ALICE_PHRASE))
}

pub(crate) fn get_account_phrase(password: &String) -> Result<String> {
    //
    Ok(String::from(crate::account::ALICE_PHRASE))
}
