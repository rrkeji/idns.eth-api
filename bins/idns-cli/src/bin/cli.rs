use anyhow::Result;
use clap::Parser;

use idns_eth_cli::{Config, ConfigBuilder};

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

fn main() -> Result<()> {
    let args = Args::parse();

    //获取配置信息
    if let Ok(config) = ConfigBuilder::default()
        .deamon(args.deamon)
        .server(args.server)
        .home(args.home)
        .build()
    {
        println!("config:{:#?}", config);
    } else {
        println!("读取配置失败!");
    }
    Ok(())
}
