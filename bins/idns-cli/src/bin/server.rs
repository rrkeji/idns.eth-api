use anyhow::{anyhow, Context, Result};
use clap::Parser;
use idns_eth_cli::server::Server;
use idns_eth_sdk::{
    idns_core::{
        account::{IdnsToken, ALICE_PHRASE, ALICE_PUBLIC_KEY},
        idns_home_path,
    },
    idns_eth_request, import_and_login, init_application, init_node, is_imported,
    login_by_password,
};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use std::{
    error::Error,
    io::{self, prelude::*, BufReader},
};

/// UDI 命令行工具
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 生成文档的路径
    #[clap(short, long, value_parser, default_value_t = String::from("~/.idns/"))]
    home: String,
    /// 本地还是远程生成
    #[clap(long, value_parser, default_value_t = true)]
    deamon: bool,
    /// 远程服务端地址
    #[clap(long, value_parser, default_value_t = String::from("http://49.232.102.140:35090"))]
    server: String,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("UDI Cli start....");

    // 解析参数
    let args = Args::parse();

    //
    init_application("663613c15a4a7198b0da88d00ae40227d1901eb7da92bb20f3f4a9329a596529")?;

    //检查是否已经导入账号
    let imported_res = is_imported().await.with_context(|| "查看是否已经导入")?;

    let token = if imported_res {
        //输入密码
        login_by_password(&String::from("123"))
            .await
            .with_context(|| format!("Failed to login_by_password {}", "123"))?
    } else {
        //输入助记词和密码
        import_and_login(&String::from(ALICE_PHRASE), &String::from("123"))
            .await
            .with_context(|| format!("Failed to import_and_login {}", "123"))?
    };

    //登录成功后初始化节点
    init_node(&token);

    fn handle_error(connection: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
        connection
            .map_err(|error| eprintln!("Incoming connection failed: {}", error))
            .ok()
    }

    let listener = LocalSocketListener::bind("/tmp/idns_cli.sock")?;
    for mut conn in listener.incoming().filter_map(handle_error) {
        println!("Incoming connection!");
        conn.write_all(b"Hello from server!\n")?;
        // Add buffering to the connection to read a line.
        let mut conn = BufReader::new(conn);
        let mut buffer = String::new();
        conn.read_line(&mut buffer)?;
        println!("Client answered: {}", buffer);
    }
    Ok(())
}
