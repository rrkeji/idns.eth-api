use anyhow::Result;
use idns_eth_core::account::IdnsToken;
use prost::Message;
use rusqlite::{
    types::ValueRef::{Blob, Integer, Null, Real, Text},
    Connection, Row,
};

use crate::types::{column::Value as IdnsValue, Column, Row as IdnsRow};

#[derive(Debug)]
pub struct FieldSchema {
    pub cid: usize,
    pub name: String,
    pub field_type: String,
    pub notnull: bool,
    pub dflt_value: Option<String>,
    pub pk: i32,
}

#[derive(Debug)]
struct SyncRowResult {
    id: i32,
    row: Result<IdnsRow>,
    cn: i32,
}

pub struct TableSync {}

impl TableSync {
    //
    pub async fn data_sync(
        conn: &Connection,
        table: &crate::sync::IdnsTableVersion,
        token: &IdnsToken,
    ) -> Result<Option<String>> {
        //查询需要同步的行

        while let Some(_) = TableSync::_get_row(conn, table, token, 10usize).await? {}
        //同步整个表

        Ok(None)
    }

    pub async fn _get_row(
        conn: &Connection,
        table: &crate::sync::IdnsTableVersion,
        token: &IdnsToken,
        batch_size: usize,
    ) -> Result<Option<()>> {
        //获取1条
        let mut stmt = conn.prepare(
            format!(
                "SELECT * FROM {} where cid ='' ORDER BY id LIMIT 0, {} ",
                table.table_name, batch_size
            )
            .as_str(),
        )?;

        let schema_iter = stmt.query_map([], |row| {
            //保存到IPFS， 获取到CID
            let idns_row = rusqlite_row_to_idns_row(row, table.col_count);

            Ok(SyncRowResult {
                id: row.get(table.id_index as usize)?,
                row: idns_row,
                cn: row.get(table.cn_index as usize)?,
            })
        })?;

        if let (_, Some(size)) = schema_iter.size_hint() {
            if size == 0usize {
                return Ok(None);
            }
        }

        for table_result in schema_iter {
            if let Ok(sync_result) = table_result {
                if let Ok(row_data) = sync_result.row {
                    //add to ipfs
                    let cid = crate::utils::ipfs_add_content(row_data.encode_to_vec()).await?;
                    //table
                    conn.execute(
                        format!(
                            "update {} set cid = ?2 where id = ?1 and _cn = ?3",
                            table.table_name
                        )
                        .as_str(),
                        (&sync_result.id, cid, &sync_result.cn),
                    )?;
                }
            }
        }
        Ok(None)
    }
}

fn rusqlite_row_to_idns_row(row: &Row<'_>, cnt: usize) -> Result<IdnsRow> {
    // let (_, Some(cnt)) = row.size_hint();

    let mut cols = Vec::<Column>::new();
    for i in 0..cnt {
        let rusqlite_value = row.get_raw(i);
        let idns_value = match rusqlite_value {
            Null => IdnsValue::NullValue(true),
            Integer(i64_v) => IdnsValue::IntegerValue(i64_v),
            Real(f64_v) => IdnsValue::RealValue(f64_v as f32),
            Text(str_v) => IdnsValue::StringValue(String::from_utf8(str_v.to_vec()).unwrap()),
            Blob(v) => IdnsValue::BlobValue(v.to_vec()),
        };
        cols.push(Column {
            value: Some(idns_value),
        });
    }

    let res = IdnsRow { cols: cols };
    return Ok(res);
}
