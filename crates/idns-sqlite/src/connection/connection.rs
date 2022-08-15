use rusqlite::Connection as RsConnection;
use std::default::Default;
use std::path::Path;
use std::str;
use std::sync::Arc;

pub use rusqlite::{Error, OpenFlags, Params, Row, Statement, ToSql};

use anyhow::{anyhow, Result};
use idns_eth_core::account::IdnsToken;

use crate::sync::Worker;

///
pub struct Connection {
    raw_connection: Arc<RsConnection>,
    sync_worker: Worker,
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

impl Drop for Connection {
    #[inline]
    fn drop(&mut self) {
        self.raw_connection.flush_prepared_statement_cache();
    }
}
///
impl Connection {
    ///
    ///
    pub fn open_with_flags(token: &IdnsToken, flags: OpenFlags) -> Result<Connection> {
        let path_str = crate::utils::get_database_path(&token.application_key)?;
        let path = Path::new(path_str.as_str());
        //
        let raw_connection = Arc::new(RsConnection::open_with_flags(&path, flags)?);

        //进行一些初始化的处理
        let sync_worker = Worker::new(token);
        sync_worker.start();

        Ok(Connection {
            raw_connection,
            sync_worker,
        })
    }
}

/// 代理rusqlite的方法
impl Connection {
    ///
    ///
    pub fn open(token: &IdnsToken) -> Result<Connection> {
        let flags = OpenFlags::default();
        Connection::open_with_flags(token, flags)
    }
    /// Convenience method to run multiple SQL statements (that cannot take any
    /// parameters).
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use idns_eth_sqlite::{Connection, Result};
    /// fn create_tables(conn: &Connection) -> Result<()> {
    ///     conn.execute_batch(
    ///         "BEGIN;
    ///          CREATE TABLE foo(x INTEGER);
    ///          CREATE TABLE bar(y TEXT);
    ///          COMMIT;",
    ///     )
    /// }
    /// ```
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    #[inline]
    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        self.raw_connection
            .execute_batch(sql)
            .map_err(|e| anyhow!("数据库执行失败:{:?}", e))
    }
    ///
    #[inline]
    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        self.raw_connection
            .execute(sql, params)
            .map_err(|e| anyhow!("数据库执行失败:{:?}", e))
    }
    ///
    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.raw_connection.path()
    }
    ///
    #[inline]
    pub fn last_insert_rowid(&self) -> i64 {
        self.raw_connection.last_insert_rowid()
    }
    ///
    #[inline]
    pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> rusqlite::Result<T>,
    {
        self.raw_connection
            .query_row(sql, params, f)
            .map_err(|e| anyhow!("数据库执行失败:{:?}", e))
    }
    ///
    #[inline]
    pub fn query_row_and_then<T, E, P, F>(&self, sql: &str, params: P, f: F) -> Result<T, E>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> Result<T, E>,
        E: From<Error>,
    {
        self.raw_connection.query_row_and_then(sql, params, f)
    }

    #[inline]
    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        self.raw_connection
            .prepare(sql)
            .map_err(|e| anyhow!("数据库执行失败:{:?}", e))
    }

    #[inline]
    pub fn close(self) -> Result<(), Error> {
        let r = self.raw_connection.flush_prepared_statement_cache();
        //TODO
        // r.map_err(move |err| err.1)
        Ok(())
    }

    #[inline]
    pub fn changes(&self) -> u64 {
        self.raw_connection.changes()
    }

    #[inline]
    pub fn is_autocommit(&self) -> bool {
        self.raw_connection.is_autocommit()
    }

    pub fn execute_named(&self, sql: &str, params: &[(&str, &dyn ToSql)]) -> Result<usize> {
        self.raw_connection
            .execute_named(sql, params)
            .map_err(|e| anyhow!("数据库执行失败:{:?}", e))
    }
}
