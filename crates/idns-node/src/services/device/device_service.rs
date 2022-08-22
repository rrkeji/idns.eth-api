use bytes::Bytes;

use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;
use tokio::runtime::Handle;

use crate::sqlite::{ipfs_add_content, ipfs_get_content};
use idns_eth_api::idns::networks::{DeviceEntity, ListDevicesResponse};
use idns_eth_api::idns::system::{BoolMessage, BytesMessage, StringMessage, U64Message};

pub struct DeviceServiceImpl;

impl DeviceServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl DeviceServiceImpl {
    pub fn list_devices(&self) -> Result<Vec<DeviceEntity>> {
        _schema()?;
        //获取conn
        let arc_conn = crate::get_connection()?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, owner_id, device_uuid, name, device_type, os_type, mac_address, category,vpnc_address,icon_url FROM devices where status = 1",
        )?;
        let mut res = Vec::<DeviceEntity>::new();

        let _iter = stmt.query_map([], |row| {
            Ok(DeviceEntity {
                id: row.get(0)?,
                owner_id: row.get(1)?,
                device_uuid: row.get(2)?,
                name: row.get(3)?,
                device_type: row.get(4)?,
                os_type: row.get(5)?,
                mac_address: row.get(6)?,
                category: row.get(7)?,
                vpnc_address: row.get(8)?,
                icon_url: row.get(9)?,
                hostname: String::new(),
                home: format!("{:?}", idns_eth_core::idns_home_path().map_err(|r| r)),
            })
        })?;
        for item in _iter {
            res.push(item?);
        }
        Ok(res)
    }

    pub fn find_by_uuid(&self, device_uuid: &String) -> Result<Option<DeviceEntity>> {
        _schema()?;
        //获取conn
        let arc_conn = crate::get_connection()?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, owner_id, device_uuid, name, device_type, os_type, mac_address, category,vpnc_address,icon_url FROM devices where device_uuid = ?1",
        )?;
        let mut res = Vec::<DeviceEntity>::new();

        let _iter = stmt.query_map([device_uuid], |row| {
            Ok(DeviceEntity {
                id: row.get(0)?,
                owner_id: row.get(1)?,
                device_uuid: row.get(2)?,
                name: row.get(3)?,
                device_type: row.get(4)?,
                os_type: row.get(5)?,
                mac_address: row.get(6)?,
                category: row.get(7)?,
                vpnc_address: row.get(8)?,
                icon_url: row.get(9)?,
                hostname: String::new(),
                home: format!("{:?}", idns_eth_core::idns_home_path().map_err(|r| r)),
            })
        })?;
        for item in _iter {
            res.push(item?);
        }
        if res.len() > 0 {
            return Ok(res.pop());
        }
        Ok(None)
    }

    pub fn find_by_mac_address(&self, mac_address: &String) -> Result<Option<DeviceEntity>> {
        _schema()?;
        //获取conn
        let arc_conn = crate::get_connection()?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, owner_id, device_uuid, name, device_type, os_type, mac_address, category,vpnc_address,icon_url FROM devices where mac_address = ?1",
        )?;
        let mut res = Vec::<DeviceEntity>::new();

        let _iter = stmt.query_map([mac_address], |row| {
            Ok(DeviceEntity {
                id: row.get(0)?,
                owner_id: row.get(1)?,
                device_uuid: row.get(2)?,
                name: row.get(3)?,
                device_type: row.get(4)?,
                os_type: row.get(5)?,
                mac_address: row.get(6)?,
                category: row.get(7)?,
                vpnc_address: row.get(8)?,
                icon_url: row.get(9)?,
                hostname: String::new(),
                home: format!("{:?}", idns_eth_core::idns_home_path().map_err(|r| r)),
            })
        })?;
        for item in _iter {
            res.push(item?);
        }
        if res.len() > 0 {
            return Ok(res.pop());
        }
        Ok(None)
    }

    pub fn list_deleted_devices(&self) -> Result<Vec<DeviceEntity>> {
        _schema()?;
        //获取conn
        let arc_conn = crate::get_connection()?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, owner_id, device_uuid, name, device_type, os_type, mac_address, category,vpnc_address,icon_url FROM devices where status = 0",
        )?;
        let mut res = Vec::<DeviceEntity>::new();

        let _iter = stmt.query_map([], |row| {
            res.push(DeviceEntity {
                id: row.get(0)?,
                owner_id: row.get(1)?,
                device_uuid: row.get(2)?,
                name: row.get(3)?,
                device_type: row.get(4)?,
                os_type: row.get(5)?,
                mac_address: row.get(6)?,
                category: row.get(7)?,
                vpnc_address: row.get(8)?,
                icon_url: row.get(9)?,
                hostname: String::new(),
                home: format!("{:?}", idns_eth_core::idns_home_path().map_err(|r| r)),
            });
            Ok(1)
        })?;
        Ok(res)
    }

    pub fn current_device_id(&self) -> Result<String> {
        Ok(super::get_device_node_id()?)
    }

    pub fn delete_device(&self, device_id: u64) -> Result<bool> {
        _schema()?;
        let arc_conn = crate::get_connection()?;
        arc_conn.execute(
            format!("UPDATE devices SET status = 0 WHERE id = {}", device_id).as_str(),
            (),
        )?;
        Ok(true)
    }

    pub fn recovery_device(&self, device_id: u64) -> Result<bool> {
        _schema()?;
        let arc_conn = crate::get_connection()?;
        arc_conn.execute(
            format!("UPDATE devices SET status = 1 WHERE id = {}", device_id).as_str(),
            (),
        )?;
        Ok(true)
    }

    pub fn create_device(&self, device: &DeviceEntity) -> Result<u64> {
        _schema()?;
        self._create_device(device)
    }

    pub fn update_device(&self, device: &DeviceEntity) -> Result<u64> {
        _schema()?;
        self._update_device(device)
    }

    fn _create_device(&self, device: &DeviceEntity) -> Result<u64> {
        let arc_conn = crate::get_connection()?;

        arc_conn.execute(
            "INSERT INTO devices (owner_id, device_uuid, name, device_type, os_type, mac_address, category,vpnc_address,icon_url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (&device.owner_id, &device.device_uuid, &device.name, &device.device_type, &device.os_type, &device.mac_address,&device.category,&device.vpnc_address,&device.icon_url),
        )?;
        Ok(1)
    }

    fn _update_device(&self, device: &DeviceEntity) -> Result<u64> {
        //
        if device.id <= 0 {
            return self._create_device(device);
        }
        let arc_conn = crate::get_connection()?;

        arc_conn.execute(
            "UPDATE devices SET owner_id = ?1, device_uuid = ?2, name = ?3, device_type = ?4, os_type = ?5, mac_address = ?6, category = ?7, vpnc_address = ?8, icon_url = ?9 WHERE id = ?10",
            (&device.owner_id, &device.device_uuid, &device.name, &device.device_type, &device.os_type, &device.mac_address,&device.category,&device.vpnc_address,&device.icon_url, device.id),
        )?;
        Ok(1)
    }
}

#[async_trait::async_trait]
impl Handler for DeviceServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.device" {
            if method_name == "list_devices" {
                //
                return response(
                    self.list_devices()
                        .map(|r| ListDevicesResponse { devices: r }),
                );
            } else if method_name == "list_deleted_devices" {
                //
                return response(
                    self.list_deleted_devices()
                        .map(|r| ListDevicesResponse { devices: r }),
                );
            } else if method_name == "find_by_uuid" {
                //
                let device_uuid = StringMessage::decode(Bytes::from(message))?;
                //
                let res = self.find_by_uuid(&device_uuid.data);
                match res {
                    Ok(Some(entity)) => {
                        return response(Ok(entity));
                    }
                    _ => {
                        return Err(Error::NormalError(-1, String::from("")));
                    }
                }
            } else if method_name == "create_device" {
                //
                let request = DeviceEntity::decode(Bytes::from(message))?;

                return response(self.create_device(&request).map(|r| U64Message { data: r }));
            } else if method_name == "delete_device" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.delete_device(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "recovery_device" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.recovery_device(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "current_device_id" {
                //
                return response(self.current_device_id().map(|r| StringMessage { data: r }));
            } else if method_name == "update_device" {
                //
                let request = DeviceEntity::decode(Bytes::from(message))?;

                return response(self.update_device(&request).map(|r| U64Message { data: r }));
            }
        }
        Err(Error::NotFoundService)
    }
}

fn _schema() -> Result<()> {
    let conn = crate::get_connection()?;
    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS devices(
        id    INTEGER PRIMARY KEY,
        owner_id    TEXT DEFAULT '',
        device_uuid     TEXT DEFAULT '',
        name        TEXT NOT NULL,
        device_type  TEXT DEFAULT 'PC',
        os_type      TEXT DEFAULT '',
        mac_address  TEXT DEFAULT '',
        category    TEXT DEFAULT '',
        vpnc_address     TEXT DEFAULT '',
        icon_url     TEXT DEFAULT '',
        status  INTEGER DEFAULT 1,
        _cid  TEXT DEFAULT '',
        _cn INTEGER DEFAULT 0,
        unique(device_uuid),
        unique(mac_address)
    );
    ",
        (),
    )?;
    Ok(())
}
