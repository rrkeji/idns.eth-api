#[macro_use]
extern crate lazy_static;

pub(crate) mod constants;
pub mod identity;
pub mod kvstore;
pub mod utils;

use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    //application key secret
    pub static ref EXTERNAL_API_IDENTITY_SIGNATURE: RwLock<Option<(String,String,String)>> = RwLock::new(None);
}

pub fn set_external_api_identity_signature(signature: (String, String, String)) -> Result<()> {
    let mut w = crate::EXTERNAL_API_IDENTITY_SIGNATURE
        .write()
        .map_err(|_err| anyhow!("获取锁(EXTERNAL_API_IDENTITY_SIGNATURE)失败"))?;
    *w = Some(signature);
    Ok(())
}

pub fn get_external_api_identity_signature() -> Result<(String, String, String)> {
    if let Some(signature) = crate::EXTERNAL_API_IDENTITY_SIGNATURE
        .read()
        .map_err(|_err| anyhow!("获取锁(EXTERNAL_API_IDENTITY_SIGNATURE)失败"))?
        .as_ref()
    {
        return Ok(signature.clone());
    } else {
        Err(anyhow!("没有signature!"))
    }
}
