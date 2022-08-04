mod schema;

pub use schema::*;

use crate::idns_core::{account::IdnsToken, idns_home_path};
use crate::sqlite::Connection;

use std::sync::Arc;

use anyhow::Result;

pub fn init_system_database(token: &IdnsToken) -> Result<()> {
    tracing::debug!("初始化系统数据库");
    //
    let storage_path = idns_home_path()?.join("sqlite");
    let _ = std::fs::create_dir_all(storage_path.as_path());
    let file_name = storage_path.join("idns.db");

    let arc_conn = Arc::new(Connection::open(file_name, token)?);

    let token = crate::get_token();
    tracing::debug!("token测试{:?}", token);

    {
        let mut w = crate::SYS_CONNECTION.write().unwrap();
        *w = Some(arc_conn.clone());
    }

    execute_schema(&arc_conn.clone())?;
    tracing::debug!("执行完成数据库shema");
    Ok(())
}

pub fn close_system_database() -> Result<()> {
    //
    {
        let mut w = crate::SYS_CONNECTION.write().unwrap();
        *w = None;
    }
    Ok(())
}
