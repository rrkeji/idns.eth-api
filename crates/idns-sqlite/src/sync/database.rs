use anyhow::Result;
use idns_eth_core::account::IdnsToken;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

use crate::sync::SchemaChecker;

#[derive(Debug)]
pub struct TableRow {
    pub schema_type: String,
    pub name: String,
    pub table_name: String,
    pub rootpage: bool,
    pub sql: String,
}

pub struct DataBaseSync {}

impl DataBaseSync {
    pub async fn data_sync(path: &Path, token: IdnsToken) -> Result<()> {
        let flags = OpenFlags::default();
        let conn = Connection::open_with_flags(path, flags).unwrap();

        //
        SchemaChecker::check(&conn, &token)?;

        tracing::debug!("=======");

        let tables = SchemaChecker::get_ctrl_table(&conn, " where sync_status = 0 ", [])?;

        for table in tables {
            //
            if let Ok(_) = conn.execute(
                "update idns_table_version set sync_status = 1 where id = ?1 and sync_status = :2",
                (table.id, 0),
            ) {
                //table
                if let Ok(cid) = crate::sync::TableSync::data_sync(&conn, &table, &token).await {
                    conn.execute(
                        "update idns_table_version set sync_status = 2, cid = ?3 where id = ?1 and sync_status = :2",
                        (table.id, 0, cid),
                    )?;
                } else {
                    conn.execute(
                        "update idns_table_version set sync_status = 0, cid = ?3 where id = ?1 and sync_status = :2",
                        (table.id, 0, ""),
                    )?;
                }
            }
        }

        Ok(())
    }
}
