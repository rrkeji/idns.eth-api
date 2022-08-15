use crate::utils::rusqlite_row_to_value;
use crate::Connection;
use anyhow::{anyhow, Result};
use rusqlite::{types::Value as SqliteValue, ToSql};
use serde_json::Value;
use std::collections::HashMap;
pub struct Queryer {}

impl Queryer {
    pub fn query(conn: &Connection, sql: &String, size: usize) -> Result<String> {
        let mut stmt = conn.prepare(sql)?;
        let schema_iter = stmt.query_map([], |row| {
            rusqlite_row_to_value(row, size).map_err(|_e| rusqlite::Error::ExecuteReturnedResults)
        })?;

        let mut result = Vec::<Value>::new();

        for table_result in schema_iter {
            if let Ok(row_value) = table_result {
                //
                result.push(Value::Array(row_value));
            }
        }
        Ok(Value::Array(result).to_string())
    }

    pub fn query_with_args(
        conn: &Connection,
        sql: &String,
        size: usize,
        args_json: &String,
    ) -> Result<String> {
        let mut stmt = conn.prepare(sql)?;

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
        let schema_iter = stmt.query_map([], |row| {
            rusqlite_row_to_value(row, size).map_err(|_e| rusqlite::Error::ExecuteReturnedResults)
        })?;

        let mut result = Vec::<Value>::new();

        for table_result in schema_iter {
            if let Ok(row_value) = table_result {
                //
                result.push(Value::Array(row_value));
            }
        }
        Ok(Value::Array(result).to_string())
    }
}
