use anyhow::{anyhow, Result};
use idns_eth_core::account::IdnsToken;
use prost::Message;
use rusqlite::{
    types::ValueRef::{Blob, Integer, Null, Real, Text},
    Connection, Row,
};

use crate::types::{column::Value as IdnsValue, Column, Row as IdnsRow};

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
        //判断是否有_cid 为空的 idns_table_version表除外
        let cnt = if table.table_name == "idns_table_version" {
            crate::utils::query_one_value::<_, usize>(
                conn,
                "SELECT COUNT(1) FROM idns_table_version WHERE _cid = '' and table_name != 'idns_table_version'",
                [],
            )?
        } else {
            crate::utils::query_one_value::<_, usize>(
                conn,
                format!("SELECT COUNT(1) FROM {} WHERE _cid = ''", table.table_name).as_str(),
                [],
            )?
        };

        if cnt == 0 {
            return Ok(None);
        }

        let table_schema_option = crate::sync::SchemaChecker::get_table_schema(
            conn,
            " where name = :1 ",
            [(&table.table_name)],
        )?;

        tracing::debug!("查询需要同步的行");

        if let Some(table_schema) = table_schema_option {
            //查询需要同步的行
            tracing::debug!("查询需要同步的行");
            let batch_size = 1000usize;
            //同步表中的行数据
            while let Some(_) =
                TableSync::_get_row(conn, table, token, batch_size, &table_schema).await?
            {
            }
            //同步整个表
            let cids: Vec<String> = vec![];

            let table = crate::types::Table {
                schema: Some(table_schema.clone()),
                max_size: batch_size as i32,
                rows: cids,
            };
            let cid = crate::utils::ipfs_add_content(table.encode_to_vec())?;
            tracing::debug!("同步表:{} Cid:{}", table_schema.table_name, cid);

            Ok(Some(cid))
        } else {
            return Err(anyhow!("没有找到表:{}", table.table_name));
        }
    }

    pub async fn _get_table(
        conn: &Connection,
        table: &crate::sync::IdnsTableVersion,
        token: &IdnsToken,
        batch_size: usize,
        table_schema: &crate::types::TableSchema,
    ) -> Result<Option<String>> {
        let mut stmt = conn.prepare(
            format!(
                "SELECT _cid FROM {} ORDER BY id LIMIT 0, {} ",
                table.table_name, batch_size
            )
            .as_str(),
        )?;

        let schema_iter = stmt.query_map([], |row| {
            //保存到IPFS， 获取到CID
            let idns_row = rusqlite_row_to_idns_row(row, table.col_count, &table);

            Ok(SyncRowResult {
                id: row.get(table.id_index as usize)?,
                row: idns_row,
                cn: row.get(table.cn_index as usize)?,
            })
        })?;
        Ok(None)
    }

    pub async fn _get_row(
        conn: &Connection,
        table: &crate::sync::IdnsTableVersion,
        token: &IdnsToken,
        batch_size: usize,
        table_schema: &crate::types::TableSchema,
    ) -> Result<Option<()>> {
        //获取1条
        let mut stmt = conn.prepare(
            format!(
                "SELECT * FROM {} where _cid ='' ORDER BY id LIMIT 0, {} ",
                table.table_name, batch_size
            )
            .as_str(),
        )?;

        let schema_iter = stmt.query_map([], |row| {
            //保存到IPFS， 获取到CID
            let idns_row = rusqlite_row_to_idns_row(row, table.col_count, &table);

            Ok(SyncRowResult {
                id: row.get(table.id_index as usize)?,
                row: idns_row,
                cn: row.get(table.cn_index as usize)?,
            })
        })?;

        for table_result in schema_iter {
            if let Ok(sync_result) = table_result {
                if let Ok(row_data) = sync_result.row {
                    //add to ipfs
                    let cid = crate::utils::ipfs_add_content(row_data.encode_to_vec())?;

                    //table
                    conn.execute(
                        format!(
                            "update {} set _cid = ?2, _cn = ABS(RANDOM() % 100000000) where id = ?1 and _cn = ?3",
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

fn rusqlite_row_to_idns_row(
    row: &Row<'_>,
    cnt: usize,
    table_schema: &crate::sync::IdnsTableVersion,
) -> Result<IdnsRow> {
    let mut cols = Vec::<Column>::new();
    for i in 0..cnt {
        let rusqlite_value = row.get_ref_unwrap(i);
        let idns_value = match rusqlite_value {
            Null => IdnsValue::NullValue(true),
            Integer(i64_v) => IdnsValue::IntegerValue(i64_v),
            Real(f64_v) => IdnsValue::RealValue(f64_v as f32),
            Text(str_v) => IdnsValue::StringValue(String::from_utf8(str_v.to_vec()).unwrap()),
            Blob(v) => IdnsValue::BlobValue(v.to_vec()),
        };
        if i == table_schema.cn_index || i == table_schema.cid_index {
            continue;
        }
        cols.push(Column {
            value: Some(idns_value),
        });
    }

    let res = IdnsRow { cols: cols };
    return Ok(res);
}
