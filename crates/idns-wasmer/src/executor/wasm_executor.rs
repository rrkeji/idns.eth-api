use anyhow::{anyhow, Result};

use idns_eth_sqlite::ipfs_get_content;
use std::sync::{Arc, Mutex};
use wasmer::{
    imports, wat2wasm, Array, Function, FunctionType, Instance, Module, Store, Type, Val, Value,
    ValueType, WasmPtr, WasmerEnv,
};
use wasmer_wasi::WasiState;

use idns_eth_core::idns_home_path;

pub struct WasmExecutor {}

impl WasmExecutor {
    //
    pub fn new() -> Self {
        Self {}
    }
}

impl WasmExecutor {
    async fn _execute(&self, instance: Instance) -> Result<String> {
        tracing::error!("idns_main");
        //prepare
        let idns_main = instance
            .exports
            .get_function("idns_main")
            .map_err(|err| anyhow!("获取函数失败:{}", err))?;

        //调用函数,获取返回结果ValueType::
        let response = idns_main
            .call(&[])
            .map_err(|err| anyhow!("执行失败:{}", err))?;

        tracing::error!("{:?}", response);
        Ok(String::new())
    }
}

impl WasmExecutor {
    pub async fn execute(&self, wasm_content_id: &String) -> Result<String> {
        //从IPFS中获取到相应的wasm内容
        let wasm_bytes = ipfs_get_content(&wasm_content_id).await?;
        // Create a Store.
        let store = Store::default();
        //创建模块
        let module =
            Module::new(&store, wasm_bytes).map_err(|err| anyhow!("创建模块失败:{}", err))?;

        let mut wasi_env = WasiState::new("idns_wasm")
            .args(&["world"])
            .env("IDNS_HOME", format!("{:?}", idns_home_path()?).as_str())
            .finalize()
            .map_err(|err| anyhow!("wasi_env:{}", err))?;

        let import_object = wasi_env
            .import_object_for_all_wasi_versions(&module)
            .map_err(|err| anyhow!("import_object:{}", err))?;
        //构造实例
        let instance = Instance::new(&module, &import_object)
            .map_err(|err| anyhow!("创建实例失败:{}", err))?;

        // 获取调用函数
        tracing::debug!("执行wasm函数");
        return self._execute(instance).await;
    }
}
