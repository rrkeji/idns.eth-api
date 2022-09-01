use anyhow::{anyhow, Result};
use idns_eth_sqlite::Connection;
use std::sync::{Arc, RwLock};

use crate::task::TaskServiceImpl;

pub enum SchedulerState {
    Init,
    Loading,
    Running,
}

pub struct Scheduler {
    state: RwLock<SchedulerState>,
}

impl Scheduler {
    //
    pub(crate) fn new() -> Self {
        Self {
            state: RwLock::new(SchedulerState::Init),
        }
    }
}

impl Scheduler {
    pub async fn loading(&self, connection: Arc<Connection>, device_uuid: &String) -> Result<()> {
        //获取写锁
        {
            let mut lock = self
                .state
                .write()
                .map_err(|err| anyhow!("获取锁失败:{}", err))?;
            *lock = SchedulerState::Loading;
        }
        //从数据库中获取到所有的任务
        let task_service = TaskServiceImpl::new(connection.clone());

        let tasks = task_service.list_tasks(device_uuid, 1)?;
        //加载任务
        //加载wasm并构造
        //更新状态
        {
            let mut lock = self
                .state
                .write()
                .map_err(|err| anyhow!("获取锁失败:{}", err))?;
            *lock = SchedulerState::Running;
        }
        Ok(())
    }
}
