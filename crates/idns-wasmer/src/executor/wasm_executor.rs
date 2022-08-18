use anyhow::Result;

use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

use idns_eth_sqlite::ipfs_get_content;

pub struct WasmExecutor {}

impl WasmExecutor {
    //
    pub fn new() -> Self {
        Self {}
    }
}

impl WasmExecutor {
    async fn _execute(&self, instance: Instance) -> Result<String> {
        //prepare
        if let Ok(prepare) = instance.exports.get_function("prepare") {
            //调用函数,获取返回结果
            if let Ok(response) = prepare.call(&[]) {
                //
            } else {
                //执行失败
            }
            //
        } else {
            //获取函数失败
        }
        //prepare
        if let Ok(start) = instance.exports.get_function("start") {
            //调用函数,获取返回结果
            if let Ok(response) = start.call(&[]) {
                //
            } else {
                //执行失败
            }
            //
        } else {
            //获取函数失败
        }
        Ok(String::new())
    }
}

impl WasmExecutor {
    pub async fn execute(&self, wasm_content_id: &String) -> Result<String> {
        //从IPFS中获取到相应的wasm内容
        let wasm_bytes = ipfs_get_content(&wasm_content_id).await?;
        // Create a Store.
        let store = Store::default();
        //
        if let Ok(module) = Module::new(&store, wasm_bytes) {
            //s
            let mut wasi_env = WasiState::new("hello")
                // .args(&["world"])
                // .env("KEY", "Value")
                .finalize()
                .unwrap();
            //
            if let Ok(import_object) = wasi_env.import_object(&module) {
                //构造实例
                if let Ok(instance) = Instance::new(&module, &import_object) {
                    // 获取调用函数
                    return self._execute(instance).await;
                } else {
                    //创建实例失败
                }
            } else {
                //导入函数失败
            }
        } else {
            //创建模块失败
        }
        Ok(String::new())
    }
}
