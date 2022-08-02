use anyhow::Result;
use idns_eth_sqlite::Connection;

pub fn execute_schema(conn: &Connection) -> Result<()> {
    conn.execute("
    CREATE TABLE IF NOT EXISTS files(
        id    INTEGER PRIMARY KEY,
        parent_id  INTEGER DEFAULT 0,
        file_name  TEXT NOT NULL,
        file_hash  TEXT NOT NULL,
        file_size  INTEGER DEFAULT 0,
        file_type  TEXT NOT NULL,
        is_dir  INTEGER DEFAULT 0,
        status  INTEGER DEFAULT 1,
        _cid  TEXT DEFAULT '',
        _cn INTEGER DEFAULT 0
    );
    ", ())?;
    Ok(())
}
