use crate::Connection;
use anyhow::{anyhow, Result};
use rusqlite::{
    params_from_iter,
    types::ValueRef::{Blob, Integer, Null, Real, Text},
    Row, ToSql,
};
use serde_json::{Number, Value};

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
}

fn rusqlite_row_to_value(row: &Row<'_>, cnt: usize) -> Result<Vec<Value>> {
    let mut cols = Vec::<Value>::new();
    for i in 0..cnt {
        let rusqlite_value = row.get_ref_unwrap(i);
        let idns_value = match rusqlite_value {
            Null => Value::Null,
            Integer(i64_v) => Value::Number(Number::from(i64_v)),
            Real(f64_v) => Value::Number(Number::from_f64(f64_v).map_or(Number::from(0i64), |r| r)),
            Text(str_v) => Value::String(String::from_utf8(str_v.to_vec()).unwrap()),
            Blob(v) => Value::Null,
        };
        cols.push(idns_value);
    }

    return Ok(cols);
}
