use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    //编译proto文件
    prost_build::compile_protos(
        &[
            "proto/networks.proto",
            "proto/account.proto",
            "proto/service.proto",
            "proto/system.proto",
            "proto/worker.proto",
        ],
        &["proto"],
    )
    .unwrap();
    Ok(())
}
