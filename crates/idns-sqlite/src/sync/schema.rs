use anyhow::{anyhow, Result};
use idns_eth_core::account::IdnsToken;
use rusqlite::{Connection, Params};
use std::ops::Deref;
use std::path::Path;

#[derive(Debug)]
pub struct IdnsTableVersion {
    pub id: i32,
    pub table_name: String,
    pub cid: String,
    pub version: i32,
    pub sync_status: i32,
    pub nonce: usize,
    pub col_count: usize,
    pub id_index: usize,
    pub cid_index: usize,
    pub cn_index: usize,
    pub _cid: String,
    pub _cn: usize,
}

pub struct SchemaChecker {}

impl SchemaChecker {
    pub fn get_table_schema<P>(
        conn: &Connection,
        where_str: &str,
        p: P,
    ) -> Result<Option<crate::types::TableSchema>>
    where
        P: Params,
    {
        let mut tables = SchemaChecker::get_table_schemas(conn, where_str, p)?;

        if tables.len() > 0 {
            Ok(tables.pop())
        } else {
            Ok(None)
        }
    }
    pub fn get_table_schemas<P>(
        conn: &Connection,
        where_str: &str,
        p: P,
    ) -> Result<Vec<crate::types::TableSchema>>
    where
        P: Params,
    {
        let mut stmt = conn.prepare(
            format!("SELECT * FROM sqlite_master {} ORDER BY name ", where_str).as_str(),
        )?;
        let schema_iter = stmt.query_map(p, |row| {
            let table_name: String = row.get(2)?;
            //
            let mut res = Vec::<crate::types::ColumnSchema>::new();

            //
            let shema = crate::types::TableSchema {
                schema_type: row.get(0)?,
                name: row.get(1)?,
                table_name: table_name,
                rootpage: row.get(3)?,
                sql: row.get(4)?,
                columns: res,
            };
            Ok(shema)
        })?;
        let mut res = Vec::<crate::types::TableSchema>::new();

        for table_result in schema_iter {
            if let Ok(mut table) = table_result {
                let table_name: String = table.table_name.clone();
                //
                let mut stmt =
                    conn.prepare(format!("PRAGMA  table_info(\"{}\")", table_name).as_str())?;
                let field_iter = stmt.query_map([], |row| {
                    Ok(crate::types::ColumnSchema {
                        column_id: row.get(0)?,
                        name: row.get(1)?,
                        column_type: row.get(2)?,
                        notnull: row.get(3)?,
                        default_value: String::from(""),
                        pk: row.get(5)?,
                    })
                })?;
                let mut cols_res = Vec::<crate::types::ColumnSchema>::new();
                for col_result in field_iter {
                    if let Ok(col) = col_result {
                        //table
                        cols_res.push(col);
                    }
                }
                table.columns = cols_res;
                //table
                res.push(table);
            }
        }
        Ok(res)
    }
    //
    pub fn get_ctrl_table<P>(
        conn: &Connection,
        where_str: &str,
        p: P,
    ) -> Result<Vec<IdnsTableVersion>>
    where
        P: Params,
    {
        tracing::debug!("get_ctrl_table:{:?}", where_str);

        let mut stmt = conn.prepare(
            format!(
                "SELECT id, table_name, cid, version, nonce, sync_status,id_index, cid_index,cn_index,col_count, _cid, _cn FROM idns_table_version {} ORDER BY id",
                where_str
            )
            .as_str(),
        )?;

        let schema_iter = stmt
            .query_map(p, |row| {
                Ok(IdnsTableVersion {
                    id: row.get(0)?,
                    table_name: row.get(1)?,
                    cid: row.get(2)?,
                    version: row.get(3)?,
                    nonce: row.get(4)?,
                    sync_status: row.get(5)?,
                    id_index: row.get(6)?,
                    cid_index: row.get(7)?,
                    cn_index: row.get(8)?,
                    col_count: row.get(9)?,
                    _cid: row.get(10)?,
                    _cn: row.get(11)?,
                })
            })
            .unwrap();

        let mut res = vec![];

        for table_result in schema_iter {
            let table = table_result?;
            res.push(table);
        }
        Ok(res)
    }

    pub fn check(conn: &Connection, token: &IdnsToken) -> Result<()> {
        //检测是否有版本控制表
        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(*) FROM sqlite_master where type ='table' and name ='{}'",
                "idns_table_version"
            )
            .as_str(),
            [],
        )?;
        if cnt == 0 {
            //不存在则进行创建
            conn.execute(
                "
                CREATE TABLE idns_table_version (
                    id    INTEGER PRIMARY KEY,
                    table_name  TEXT DEFAULT '',
                    version INTEGER DEFAULT 0,
                    sync_status INTEGER DEFAULT 0,
                    id_index INTEGER DEFAULT 0,
                    cid_index INTEGER DEFAULT 0,
                    cn_index INTEGER DEFAULT 0,
                    col_count INTEGER DEFAULT 0,
                    cid TEXT DEFAULT '',
                    nonce INTEGER DEFAULT 0,
                    _cid  TEXT DEFAULT '',
                    _cn INTEGER DEFAULT 0
                );",
                (), // empty list of parameters.
            )?;
        }

        //判断每个表的schema
        let schemas = SchemaChecker::get_table_schemas(conn, " where type = 'table' ", [])?;

        for table in schemas {
            if table.schema_type == "table" {
                SchemaChecker::create_trigger(&conn, &table)?;
            }
        }
        Ok(())
    }

    pub fn create_trigger(conn: &Connection, table: &crate::types::TableSchema) -> Result<()> {
        let table_name = table.table_name.clone();
        //判断是否有 cid cn
        let (has, id_index, cid_index, cn_index, count) = SchemaChecker::has_sync_field(table)?;
        if !has {
            return Ok(());
        }
        //
        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(*) FROM sqlite_master where type ='trigger' and name ='trigger__cid_update_{}'",
                table_name
            )
            .as_str(),
            [],
        )?;
        if cnt == 0 {
            if table_name != "idns_table_version" {
                //
                conn.execute(
                    format!(
                        "CREATE  TRIGGER trigger__cid_update_{}  AFTER UPDATE ON {}  for each row
                    BEGIN
                        update {} set _cid = \"\", _cn = ABS(RANDOM() % 100000000)   where id = new.id and new._cid = old._cid and new._cid != '';
                        update idns_table_version set nonce = ABS(RANDOM() % 100000000) where table_name = '{}';
                    END;",
                        table_name, table_name, table_name, table_name
                    )
                    .as_str(),
                    (), // empty list of parameters.
                )?;
            } else {
                //
                conn.execute(
                    format!(
                        "CREATE  TRIGGER trigger__cid_update_{}  AFTER UPDATE OF cid ON {}  for each row
                    BEGIN
                        update {} set _cid = \"\", _cn = ABS(RANDOM() % 100000000)   where id = new.id and new._cid = old._cid and new._cid != '';
                    END;",
                        table_name, table_name, table_name
                    )
                    .as_str(),
                    (), // empty list of parameters.
                )?;
            }
        }

        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(*) FROM sqlite_master where type ='trigger' and name ='trigger__cid_insert_{}'",
                table_name
            )
            .as_str(),
            [],
        )?;

        if cnt == 0 {
            if table_name != "idns_table_version" {
                //
                conn.execute(
                    format!(
                        "
                    CREATE  TRIGGER trigger__cid_insert_{}  AFTER INSERT ON {}
                    BEGIN
                        update {} set _cid = \"\", _cn =ABS(RANDOM() % 100000000)   where id = new.id;
                        update idns_table_version set nonce = ABS(RANDOM() % 100000000) where table_name = '{}';
                    END;",
                        table_name, table_name, table_name, table_name
                    )
                    .as_str(),
                    (), // empty list of parameters.
                )?;
            } else {
            }
        }
        // insert idns_table_version
        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(*) FROM idns_table_version where table_name ='{}'",
                table_name
            )
            .as_str(),
            [],
        )?;
        if cnt == 0 {
            conn.execute(
                "INSERT INTO idns_table_version (table_name, cid, id_index, cid_index, cn_index, col_count) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (table_name, "", id_index, cid_index, cn_index,count),
            )?;
        } else {
            conn.execute(
                "UPDATE idns_table_version SET id_index = ?2, cid_index = ?3, cn_index = ?4, col_count = ?5 where table_name = ?1",
                (table_name, id_index, cid_index, cn_index,count),
            )?;
        }

        Ok(())
    }

    pub fn create_table_and_trigger(
        conn: &Connection,
        table_name: &str,
        table_ddl: &str,
    ) -> Result<()> {
        let cnt = crate::utils::query_one_value::<_, i32>(
            conn,
            format!(
                "SELECT COUNT(*) FROM sqlite_master where type ='table' and name ='{}'",
                table_name
            )
            .as_str(),
            [],
        )?;

        if cnt > 0 {
            //表已经存在
            return Err(anyhow!("表已经存在"));
        }
        conn.execute(
            table_ddl,
            (), // empty list of parameters.
        )?;

        //查询一条
        let tables = SchemaChecker::get_table_schemas(
            conn,
            format!(" where name = '{}' ", table_name).as_str(),
            [],
        )?;

        if tables.len() == 1 {
            SchemaChecker::create_trigger(conn, tables.get(0).unwrap())?;
        }
        Ok(())
    }

    /// Returns (has,id_index,cid_index,cn_index,count)
    pub fn has_sync_field(
        table: &crate::types::TableSchema,
    ) -> Result<(bool, usize, usize, usize, usize)> {
        let mut has_cid = false;
        let mut has_cn = false;
        let mut has_id = false;

        let mut id_index: usize = 0usize;
        let mut cid_index: usize = 0usize;
        let mut cn_index: usize = 0usize;

        let mut size = 0usize;

        let columns = &table.columns;
        for field in columns {
            size = size + 1;
            if field.name == "id" {
                has_id = true;
                id_index = field.column_id as usize;
            }
            if field.name == "_cid" {
                has_cid = true;
                cid_index = field.column_id as usize;
            }
            if field.name == "_cn" {
                has_cn = true;
                cn_index = field.column_id as usize;
            }
        }

        Ok((
            has_cid && has_cn && has_id,
            id_index,
            cid_index,
            cn_index,
            size,
        ))
    }
}
