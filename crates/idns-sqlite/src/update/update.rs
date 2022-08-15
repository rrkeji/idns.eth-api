use crate::Connection;
use anyhow::Result;
use rusqlite::{types::Value as SqliteValue, ToSql};
use serde_json::{Number, Value};
use std::collections::HashMap;

pub struct Updater {}

impl Updater {
    ///带参数更新
    pub fn update_with_args(
        arc_conn: &Connection,
        sql: &String,
        args_json: &String,
    ) -> Result<usize> {
        let mut args_sqlite_values = HashMap::<String, SqliteValue>::new();
        let mut named_args: Vec<(&str, &dyn ToSql)> = vec![];

        if let Value::Object(json_value) = serde_json::from_str(args_json.as_str())? {
            for (k, v) in json_value {
                args_sqlite_values.insert(k.clone(), crate::utils::value_to_rusqlite_value(&v)?);
                //
            }
        }
        for (k, v) in &args_sqlite_values {
            named_args.push((k, v as &dyn ToSql));
        }
        let res = arc_conn.execute_named(sql.as_str(), &named_args)?;
        return Ok(res);
    }
    ///执行sql进行更新
    pub fn update(arc_conn: &Connection, sql: &String) -> Result<usize> {
        let res = arc_conn.execute(sql.as_str(), [])?;
        return Ok(res);
    }
}
