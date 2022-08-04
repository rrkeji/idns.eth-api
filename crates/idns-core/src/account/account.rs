use anyhow::{anyhow, Result};
use bip39::{Language, Mnemonic, MnemonicType};
use sp_core::{hexdisplay::HexDisplay, Pair};

pub struct Account;

impl Account {
    ///生成一个账号,主要是生成phrase
    ///
    /// 返回,(public_key, phrase)
    pub fn generate_account() -> Result<(String, String)> {
        //生成account
        let words = MnemonicType::Words12;
        let mnemonic = Mnemonic::new(words, Language::English);
        let secret_phrase = mnemonic.phrase();
        //生成
        if let Ok((pair, _seed)) = sp_core::sr25519::Pair::from_phrase(secret_phrase, None) {
            let public_key_object = pair.public();
            let public_key = format!("{}", HexDisplay::from(&public_key_object.as_ref()));
            Ok((public_key.clone(), String::from(secret_phrase)))
        } else {
            Err(anyhow!("Fail!"))
        }
    }
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_account() {
        //
        let _ = Account::generate_account().unwrap();
    }
}
