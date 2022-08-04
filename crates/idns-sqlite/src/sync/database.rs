use crate::sync::{IdnsTableVersion, SchemaChecker};
use crate::types::{TableCidPair, TablesArray};
use anyhow::{anyhow, Result};
use bytes::Bytes;
use idns_eth_core::account::IdnsToken;
use idns_eth_core::kvstore::KVStore;
use prost::Message;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::sync::RwLock;

lazy_static! {
    static ref DATA_UPLOAD_LOCK: RwLock<bool> = RwLock::new(false);
    static ref DATA_DOWNLOAD_LOCK: RwLock<bool> = RwLock::new(false);
}

pub struct DataBaseSync {}

impl DataBaseSync {
    pub async fn data_sync(token: &IdnsToken) -> Result<()> {
        //如果正在执行则退出
        let path_str = crate::utils::get_database_path(&token.application_key)?;
        let path = Path::new(path_str.as_str());

        let flags = OpenFlags::default();
        let conn = Connection::open_with_flags(path, flags).unwrap();

        //获取线上的数据版本以及CID
        let (online_cid, online_version) = DataBaseSync::download(&conn, token).await?;
        tracing::debug!("获取线上版本完成{} {}!", online_version, online_cid);
        //
        tracing::debug!("检查数据的schema定义数据!");
        SchemaChecker::check(&conn, &token)?;

        tracing::debug!("获取所有的版本控制表!");
        let mut tables: Vec<IdnsTableVersion> =
            SchemaChecker::get_ctrl_table(&conn, " where cid = '' ", [])?;

        let mut all_tables: Vec<IdnsTableVersion> = vec![];
        let mut changed = false;

        while tables.len() > 0 {
            //同步所有表的数据
            for table in tables {
                tracing::debug!("同步{}表", table.table_name);
                //table
                if let Ok(cid_option) =
                    crate::sync::TableSync::data_upload(&conn, &table, &token).await
                {
                    if let Some(cid) = cid_option {
                        conn.execute(
                            "update idns_table_version set cid = ?2 where id = ?1 ",
                            (table.id, cid),
                        )?;
                        changed = true;
                    }
                }
            }
            //再次获取
            all_tables = SchemaChecker::get_ctrl_table(&conn, " where cid != '' ", [])?;
            tables = SchemaChecker::get_ctrl_table(&conn, " where cid = '' ", [])?;
        }

        if !changed {
            //没有变化
            tracing::debug!("数据没有变化!");
            return Ok(());
        }

        //整个库的内容备份
        let table_array = TablesArray {
            tables: all_tables
                .iter()
                .map(|table| TableCidPair {
                    table_name: table.table_name.clone(),
                    cid: table.cid.clone(),
                })
                .collect(),
        };
        let cid = crate::utils::ipfs_add_content(table_array.encode_to_vec())?;
        tracing::debug!("同步数据库:Cid:{}", cid);
        //提交到线上
        let (latest_online_cid, latest_online_version) = KVStore::get_value(token).await?;

        if latest_online_version != online_version || latest_online_cid != online_cid {
            //线上有新的版本,本地同步失败,继续下载同步
            return Err(anyhow!("线上有新的版本,本地同步失败,继续下载同步"));
        }
        KVStore::set_value(token, latest_online_version, &cid).await?;
        Ok(())
    }

    ///如果本地没有的时候，下载完整的数据库数据
    pub async fn download(conn: &Connection, token: &IdnsToken) -> Result<(String, i64)> {
        //
        SchemaChecker::create_ctrl_table(&conn)?;
        //删除版本控制索引
        SchemaChecker::drop_version_trigger(&conn)?;

        //获取到根的CID
        (|| async {
            let (root_cid, version) = KVStore::get_value(token).await?;

            let data = crate::utils::ipfs_get_content(&root_cid).await?;
            //获取所有表的hash
            let tables = crate::types::TablesArray::decode(Bytes::from(data))?;
            for table in tables.tables {
                tracing::debug!("下载表:{:?}", &table);
                //和数据库中的hash进行比较
                let cnt = crate::utils::query_one_value::<_, usize>(
                    conn,
                    "SELECT COUNT(1) FROM idns_table_version WHERE cid = ?1 and table_name = ?2;",
                    [&table.cid, &table.table_name],
                )?;

                if cnt == 1 {
                    continue;
                }
                //比较每个表
                crate::sync::TableSync::data_download(&conn, &table.cid, token).await?;
                conn.execute(
                    "update idns_table_version set cid = ?1 where table_name = ?2 ",
                    [&table.cid, &table.table_name],
                )?;
            }
            Ok((root_cid, version))
        })()
        .await
        .map_err(|e| {
            //恢复触发器 TODO 这个时间间隙插入的数据
            let _ = SchemaChecker::create_version_trigger(&conn);
            e
        })
    }
}
