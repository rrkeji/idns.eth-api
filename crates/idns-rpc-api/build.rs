use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    //编译proto文件
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("idns_rpc_descriptor.bin"))
        .compile(&["rpc.proto"], &["proto"])
        .unwrap();
    Ok(())
}
