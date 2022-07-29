use anyhow::Result;
use idns_eth_core::account::IdnsToken;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

use crate::sync::SchemaChecker;

pub struct DataBaseSync {}

impl DataBaseSync {
    pub async fn data_sync(path: &Path, token: &IdnsToken) -> Result<()> {
        //如果正在执行则退出
        let flags = OpenFlags::default();
        let conn = Connection::open_with_flags(path, flags).unwrap();

        //
        tracing::debug!("检查数据的schema定义数据!");
        SchemaChecker::check(&conn, &token)?;

        let cnt = crate::utils::query_one_value::<_, i32>(&conn, "SELECT COUNT(1) FROM idns_table_version WHERE table_name = 'idns_table_version' and sync_status = 1 ", [])?;

        if cnt == 1 {
            tracing::debug!("任务正在执行!");
            //TODO 超时
            return Ok(());
        }
        if let Err(err) = conn.execute(
            "update idns_table_version set sync_status = 1 where table_name = 'idns_table_version' ",
            [],
        ) {
            tracing::debug!("更新任务的状态失败{}!",err);
            //TODO
            return Ok(());
        }

        tracing::debug!("获取所有的版本控制表!");
        let tables = SchemaChecker::get_ctrl_table(
            &conn,
            " where sync_status = 0 or table_name = 'idns_table_version'  ",
            [],
        )?;

        let mut version_schema: Option<crate::sync::schema::IdnsTableVersion> = None;

        //同步所有表的数据
        for table in tables {
            if table.table_name == "idns_table_version" {
                version_schema = Some(table);
                continue;
            }
            tracing::debug!("同步{}表", table.table_name);
            //
            if let Err(err) = conn.execute(
                "update idns_table_version set sync_status = 1 where id = ?1 and sync_status = ?2 ",
                (table.id, "0"),
            ) {
                tracing::debug!("同步{}表抛错:{:?}", table.table_name, err);
            } else {
                //table
                if let Ok(cid_option) =
                    crate::sync::TableSync::data_sync(&conn, &table, &token).await
                {
                    if let Some(cid) = cid_option {
                        conn.execute(
                            "update idns_table_version set sync_status = 2, cid = ?3 where id = ?1 and sync_status = ?2",
                            (table.id, "1", cid),
                        )?;
                        continue;
                    }
                }
                conn.execute(
                        "update idns_table_version set sync_status = 2  where id = ?1 and sync_status = ?2",
                        (table.id, "1"),
                    )?;
            }
        }
        //整个库的内容备份
        if let Some(ver_schema) = version_schema {
            if let Ok(cid_option) =
                crate::sync::TableSync::data_sync(&conn, &ver_schema, &token).await
            {
                if let Some(cid) = cid_option {
                    //数据有变化
                    tracing::debug!("Root Cid:{:?}", cid);
                    conn.execute(
                        "update idns_table_version set sync_status = 0, cid = ?2 where table_name =  ?1 ",
                        ("idns_table_version", cid),
                    )?;
                    //保存CID到kvstore中

                    return Ok(());
                }
                //数据没有变化
            }
        }
        conn.execute(
            "update idns_table_version set sync_status = 0  where table_name = 'idns_table_version'",
            [],
        )?;
        Ok(())
    }
}
