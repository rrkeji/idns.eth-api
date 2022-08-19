use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};

use idns_eth_api::idns::wasmer::TaskEntity;
use idns_eth_sqlite::Connection;
use std::sync::Arc;

pub struct TaskServiceImpl {
    connection: Arc<Connection>,
}

impl TaskServiceImpl {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl TaskServiceImpl {
    pub fn list_tasks(&self, device_uuid: &String) -> Result<Vec<TaskEntity>> {
        let arc_conn = self.connection.clone();
        _schema(&arc_conn)?;
        //获取conn
        let mut stmt = arc_conn.prepare(
            "SELECT id, owner_id, wasm_cid, name, icon_url, gas, target_device, trade_no,target_os_type,category FROM tasks where target_device = ?1 and status = 1",
        )?;
        let mut res = Vec::<TaskEntity>::new();

        let _iter = stmt.query_map([device_uuid], |row| {
            Ok(TaskEntity {
                id: row.get(0)?,
                owner_id: row.get(1)?,
                wasm_cid: row.get(2)?,
                name: row.get(3)?,
                icon_url: row.get(4)?,
                gas: row.get(5)?,
                target_device: row.get(6)?,
                trade_no: row.get(7)?,
                target_os_type: row.get(8)?,
                category: row.get(9)?,
            })
        })?;
        for item in _iter {
            res.push(item?);
        }
        Ok(res)
    }

    pub fn list_deleted_tasks(&self, device_uuid: &String) -> Result<Vec<TaskEntity>> {
        let arc_conn = self.connection.clone();
        _schema(&arc_conn)?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, owner_id, wasm_cid, name, icon_url, gas, target_device, trade_no,target_os_type,category FROM tasks where target_device = ?1 and status = 0",
        )?;
        let mut res = Vec::<TaskEntity>::new();

        let _iter = stmt.query_map([device_uuid], |row| {
            res.push(TaskEntity {
                id: row.get(0)?,
                owner_id: row.get(1)?,
                wasm_cid: row.get(2)?,
                name: row.get(3)?,
                icon_url: row.get(4)?,
                gas: row.get(5)?,
                target_device: row.get(6)?,
                trade_no: row.get(7)?,
                target_os_type: row.get(8)?,
                category: row.get(9)?,
            });
            Ok(1)
        })?;
        Ok(res)
    }

    pub fn delete_task(&self, task_id: u64) -> Result<bool> {
        let arc_conn = self.connection.clone();
        _schema(&arc_conn)?;
        arc_conn.execute(
            format!("UPDATE tasks SET status = 0 WHERE id = {}", task_id).as_str(),
            (),
        )?;
        Ok(true)
    }

    pub fn recovery_task(&self, task_id: u64) -> Result<bool> {
        let arc_conn = self.connection.clone();
        _schema(&arc_conn)?;
        arc_conn.execute(
            format!("UPDATE tasks SET status = 1 WHERE id = {}", task_id).as_str(),
            (),
        )?;
        Ok(true)
    }

    pub fn create_task(&self, task: &TaskEntity) -> Result<u64> {
        let arc_conn = self.connection.clone();
        _schema(&arc_conn)?;
        self._create_task(task)
    }

    pub fn update_task(&self, task: &TaskEntity) -> Result<u64> {
        let arc_conn = self.connection.clone();
        _schema(&arc_conn)?;
        self._update_task(task)
    }

    fn _create_task(&self, task: &TaskEntity) -> Result<u64> {
        let arc_conn = self.connection.clone();

        arc_conn.execute(
            "INSERT INTO devices (owner_id, wasm_cid, name, icon_url, gas, target_device, trade_no,target_os_type,category) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (&task.owner_id, &task.wasm_cid, &task.name, &task.icon_url, &task.gas, &task.target_device,&task.trade_no,&task.target_os_type,&task.category),
        )?;
        Ok(1)
    }

    fn _update_task(&self, task: &TaskEntity) -> Result<u64> {
        //
        if task.id <= 0 {
            return self._create_task(task);
        }
        let arc_conn = self.connection.clone();

        arc_conn.execute(
            "UPDATE devices SET owner_id = ?1, wasm_cid = ?2, name = ?3, icon_url = ?4, gas = ?5, target_device = ?6, trade_no = ?7, target_os_type = ?8, category = ?9 WHERE id = ?10",
            (&task.owner_id, &task.wasm_cid, &task.name, &task.icon_url, &task.gas, &task.target_device,&task.trade_no,&task.target_os_type,&task.category, task.id),
        )?;
        Ok(1)
    }
}

fn _schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS tasks(
        id    INTEGER PRIMARY KEY,
        owner_id     TEXT DEFAULT '',
        wasm_cid     TEXT DEFAULT '',
        name        TEXT NOT NULL,
        icon_url     TEXT DEFAULT '',
        gas     INTEGER DEFAULT 0,
		target_device     TEXT DEFAULT '',
        trade_no  TEXT DEFAULT '',
        target_os_type      TEXT DEFAULT '',
        category    TEXT DEFAULT '',
        status  INTEGER DEFAULT 1,
        _cid  TEXT DEFAULT '',
        _cn INTEGER DEFAULT 0
    );
    ",
        (),
    )?;
    Ok(())
}
