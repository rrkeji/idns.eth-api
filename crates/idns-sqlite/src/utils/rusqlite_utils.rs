use anyhow::{anyhow, Result};
use rusqlite::{types::FromSql, Connection, Params};

pub fn query_one_value<P, V>(connection: &Connection, sql: &str, p: P) -> Result<V>
where
    P: Params,
    V: FromSql,
{
    let mut stmt = connection.prepare(sql)?;

    let result_iter = stmt.query_map(p, |row| Ok(row.get(0)?))?;

    for result in result_iter {
        if let Ok(i32_temp) = result {
            //table
            return Ok(i32_temp);
        }
    }
    Err(anyhow!(""))
}
