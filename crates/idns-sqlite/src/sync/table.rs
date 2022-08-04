use crate::types::{
    column::Value as IdnsValue, Column, IdCidPair, Row as IdnsRow, RowsArray, TableRowsHash,
    TableSchema,
};
use anyhow::{anyhow, Result};
use bytes::Bytes;
use idns_eth_core::account::IdnsToken;
use prost::Message;
use rusqlite::{
    params_from_iter,
    types::Value,
    types::ValueRef::{Blob, Integer, Null, Real, Text},
    Connection, Row, ToSql,
};
use std::collections::HashMap;

#[derive(Debug)]
struct SyncRowResult {
    id: i32,
    row: Result<IdnsRow>,
    cn: i32,
}

pub struct TableSync {}

impl TableSync {
    //
    pub async fn data_upload(
        conn: &Connection,
        table: &crate::sync::IdnsTableVersion,
        token: &IdnsToken,
    ) -> Result<Option<String>> {
        //判断是否有_cid 为空的 idns_table_version表除外
        let cnt = crate::utils::query_one_value::<_, usize>(
            conn,
            format!("SELECT COUNT(1) FROM {} WHERE _cid = ''", table.table_name).as_str(),
            [],
        )?;

        let table_schema_option = crate::sync::SchemaChecker::get_table_schema(
            conn,
            " where name = :1 ",
            [(&table.table_name)],
        )?;

        if let Some(table_schema) = table_schema_option {
            //查询需要同步的行
            tracing::debug!("查询需要同步的行");
            let batch_size = 1000u32;
            //同步表中的行数据
            let mut offset = 0u32;
            while let Some(_) = TableSync::_get_row(conn, table, token, offset, batch_size).await? {
                offset = offset + batch_size;
            }
            //同步整个表
            let mut cids: Vec<TableRowsHash> = vec![];
            //按照batch_size进行分页
            offset = 0u32;
            while let Some(pcid) =
                TableSync::_get_table(conn, table, token, offset, batch_size).await?
            {
                cids.push(TableRowsHash {
                    offset: offset,
                    size: batch_size,
                    cid: pcid,
                });
                offset = offset + batch_size;
            }
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
        offset: u32,
        batch_size: u32,
    ) -> Result<Option<String>> {
        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(1) FROM (SELECT id FROM {} ORDER BY id LIMIT {}, {} ) a",
                table.table_name, offset, batch_size
            )
            .as_str(),
            [],
        )?;
        if cnt <= 0 {
            return Ok(None);
        }

        let mut stmt = conn.prepare(
            format!(
                "SELECT id, _cid FROM {} ORDER BY id LIMIT {}, {} ",
                table.table_name, offset, batch_size
            )
            .as_str(),
        )?;
        let schema_iter = stmt.query_map([], |row| {
            //保存到IPFS， 获取到CID
            let id: u64 = row.get(0)?;
            let cid: String = row.get(1)?;
            Ok((id, cid))
        })?;

        let mut cids: Vec<IdCidPair> = vec![];
        for table_result in schema_iter {
            let (id, cid) = table_result?;
            cids.push(IdCidPair { id: id, cid: cid });
        }
        //
        let v = RowsArray { rows: cids };

        let cid = crate::utils::ipfs_add_content(v.encode_to_vec())?;

        //INSERT INTO idns_rows_version(table_name, offset, size, cid) VALUES (?1, ?2, ?3, ?4) ON CONFLICT (table_name, offset, size) DO UPDATE SET cid= ?5;
        conn.execute("INSERT INTO idns_rows_version(table_name, offset, size, cid) VALUES (?1, ?2, ?3, ?4) ON CONFLICT (table_name, offset, size) DO UPDATE SET cid= ?5;", 
                    (&table.table_name, offset, batch_size, &cid,&cid))?;

        Ok(Some(cid))
    }

    pub async fn _get_row(
        conn: &Connection,
        table: &crate::sync::IdnsTableVersion,
        token: &IdnsToken,
        offset: u32,
        batch_size: u32,
    ) -> Result<Option<()>> {
        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(1) FROM (SELECT id FROM {} where _cid ='' ORDER BY id LIMIT {}, {} ) a",
                table.table_name, offset, batch_size
            )
            .as_str(),
            [],
        )?;
        if cnt <= 0 {
            return Ok(None);
        }

        //获取1条
        let mut stmt = conn.prepare(
            format!(
                "SELECT * FROM {} where _cid ='' ORDER BY id LIMIT {}, {} ",
                table.table_name, offset, batch_size
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
        Ok(Some(()))
    }

    pub async fn data_download(conn: &Connection, cid: &String, token: &IdnsToken) -> Result<()> {
        //
        //
        let data = crate::utils::ipfs_get_content(&cid).await?;
        let table = crate::types::Table::decode(Bytes::from(data))?;

        tracing::debug!("Table:{:?}", table);
        let mut table_name = String::new();
        //schema
        if let Some(schema) = table.schema {
            let _ = conn.execute(&schema.sql, []);

            table_name = schema.table_name.clone();

            let batch_size = &table.max_size;
            //Row rows 批量的行的hash值比较
            let rows = &table.rows;
            //Vec<TableRowsHash>
            for row_hash in rows {
                // offset: offset, size: batch_size, cid: pcid,
                //和数据库中的hash进行比较
                let cnt = crate::utils::query_one_value::<_, usize>(
                conn,
                "SELECT COUNT(1) FROM idns_rows_version WHERE table_name = ?1 and offset = ?2 and size = ?3 and cid = ?4;",
                (&table_name, row_hash.offset, row_hash.size, &row_hash.cid),
            )?;

                if cnt == 1 {
                    continue;
                }
                //比较行
                _download_table_rows(conn, &row_hash.cid, token, &schema).await?;

                conn.execute("INSERT INTO idns_rows_version(table_name, offset, size, cid) VALUES (?1, ?2, ?3, ?4) ON CONFLICT (table_name, offset, size) DO UPDATE SET cid= ?5;", 
                    (&table_name, row_hash.offset, batch_size, &cid,&cid))?;
                //
            }
        }

        Ok(())
    }
}

async fn _download_table_rows(
    conn: &Connection,
    cid: &String,
    token: &IdnsToken,
    table_schema: &TableSchema,
) -> Result<()> {
    //
    let data = crate::utils::ipfs_get_content(&cid).await?;
    let rows_array = RowsArray::decode(Bytes::from(data))?;
    let table_name = table_schema.table_name.clone();

    tracing::debug!("rows_array:{:?}", rows_array);

    for row in rows_array.rows {
        //IdCidPair
        let id = row.id;
        let cid = row.cid.clone();

        let cnt = crate::utils::query_one_value::<_, usize>(
            conn,
            format!(
                "SELECT COUNT(1) FROM {} WHERE id = ?1 and _cid = ?2;",
                &table_name
            )
            .as_str(),
            (id, &cid),
        )?;

        if cnt == 1 {
            continue;
        }
        //更新行
        //SyncRowResult
        _download_table_row(conn, token, id, &cid, table_schema).await?;
    }

    Ok(())
}

async fn _download_table_row(
    conn: &Connection,
    _token: &IdnsToken,
    id: u64,
    cid: &String,
    table_schema: &TableSchema,
) -> Result<()> {
    //
    let data = crate::utils::ipfs_get_content(&cid).await?;
    let mut row_result = IdnsRow::decode(Bytes::from(data))?;
    //IdnsRow  Column cols
    let insert_sql = table_schema.insert_sql.clone();
    row_result.cols.push(Column {
        value: Some(IdnsValue::StringValue(cid.clone())),
    });
    row_result.cols.push(Column {
        value: Some(IdnsValue::IntegerValue(id as i64)),
    });

    let args = idns_row_to_rusqlite_row(&row_result.cols)?;

    tracing::debug!("update sql:{:?} args:{:?}", insert_sql, args);

    let columns = &table_schema.columns;
    let mut named_args: Vec<(&str, &dyn ToSql)> = vec![];
    let mut named_temp_map: HashMap<usize, String> = HashMap::new();
    let cid_value = Value::Text(cid.clone());

    for field in columns {
        let name = format!(":{}", &field.name);
        named_temp_map.insert(field.column_id as usize, name);
    }
    for field in columns {
        if (field.name != "_cid") {
            named_args.push((
                named_temp_map
                    .get(&(field.column_id as usize))
                    .unwrap()
                    .as_str(),
                args.get(field.column_id as usize).unwrap() as &dyn ToSql,
            ));
        } else {
            named_args.push((
                named_temp_map
                    .get(&(field.column_id as usize))
                    .unwrap()
                    .as_str(),
                &cid_value as &dyn ToSql,
            ));
        }
    }
    //&[(&str, &dyn ToSql)]
    let _ = conn.execute_named(insert_sql.as_str(), &named_args)?;
    Ok(())
}

fn idns_row_to_rusqlite_row(cols: &Vec<Column>) -> Result<Vec<Value>> {
    Ok(cols
        .iter()
        .map(|col| match &col.value {
            Some(v) => match v {
                IdnsValue::NullValue(_) => Value::Null,
                IdnsValue::IntegerValue(v) => Value::Integer(*v),
                IdnsValue::RealValue(v) => Value::Real(*v as f64),
                IdnsValue::BlobValue(v) => Value::Blob(v.to_vec()),
                IdnsValue::StringValue(v) => Value::Text(v.clone()),
            },
            _ => Value::Null,
        })
        .collect())
}

fn rusqlite_row_to_idns_row(
    row: &Row<'_>,
    cnt: usize,
    _table_schema: &crate::sync::IdnsTableVersion,
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
        cols.push(Column {
            value: Some(idns_value),
        });
    }

    let res = IdnsRow { cols: cols };
    return Ok(res);
}
