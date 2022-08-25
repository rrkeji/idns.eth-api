use crate::Config;
use anyhow::{Context, Result};
use idns_eth_sdk::{
    idns_core::account::{ALICE_PHRASE, ALICE_PUBLIC_KEY},
    import_and_login, init_application, is_imported, login_by_password,
};
use prost::Message;
use std::collections::HashMap;

#[derive(Default, Builder, Debug)]
pub struct Server {
    // pub config: Config,
}

impl Server {
    pub fn start(&self) -> Result<()> {
        //
        // init_application("663613c15a4a7198b0da88d00ae40227d1901eb7da92bb20f3f4a9329a596529");

        // //检查是否已经导入账号
        // let is_imported = is_imported().with_context(|| "")?;

        // let token = if is_import {
        //     //输入密码
        //     login_by_password(&String::from("123"))
        //         .with_context(|| format!("Failed to login_by_password {}", "123"))?
        // } else {
        //     //输入助记词和密码
        //     import_and_login(&String::from(ALICE_PHRASE), &String::from("123"))
        //         .with_context(|| format!("Failed to import_and_login {}", "123"))?
        // };

        //登录成功
        Ok(())
    }

    pub fn request(command: &String) -> Result<String> {
        Ok(String::new())
    }
}
