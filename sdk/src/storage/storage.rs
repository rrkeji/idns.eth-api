use bytes::Bytes;

use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;
use tokio::runtime::Handle;

use crate::sqlite::{ipfs_add_content, ipfs_get_content};
use idns_eth_api::idns::storage::{
    FileEntity, ListFilesRequest, ListFilesResponse, MkdirRequest, MkdirResponse,
};
use idns_eth_api::idns::system::{BoolMessage, BytesMessage, StringMessage, U64Message};

pub struct StorageServiceImpl;

impl StorageServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl StorageServiceImpl {
    ///创建目录
    pub fn mkdir(&self, parent_id: u64, file_name: &String) -> Result<bool> {
        let file_entity = FileEntity {
            id: 0,
            parent_id: parent_id,
            file_name: file_name.clone(),
            file_hash: String::new(),
            file_size: 0,
            file_type: String::from("DIR"),
            is_dir: true,
        };
        let _ = &self._create_file(&file_entity)?;
        Ok(true)
    }

    pub fn list_files(&self, root_id: u64) -> Result<Vec<FileEntity>> {
        //获取conn
        let arc_conn = crate::get_connection()?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, parent_id, file_name, file_hash, file_size, file_type, is_dir FROM files where parent_id = ?1 and status = 1",
        )?;
        let mut res = Vec::<FileEntity>::new();

        let _iter = stmt.query_map([root_id], |row| {
            let is_dir_int: i32 = row.get(6)?;
            Ok(FileEntity {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                file_name: row.get(2)?,
                file_hash: row.get(3)?,
                file_size: row.get(4)?,
                file_type: row.get(5)?,
                is_dir: if is_dir_int == 1 { true } else { false },
            })
        })?;
        for item in _iter {
            res.push(item?);
        }
        Ok(res)
    }

    pub fn list_deleted_files(&self) -> Result<Vec<FileEntity>> {
        //获取conn
        let arc_conn = crate::get_connection()?;
        let mut stmt = arc_conn.prepare(
            "SELECT id, parent_id, file_name, file_hash, file_size, file_type, is_dir FROM files where status = 0",
        )?;
        let mut res = Vec::<FileEntity>::new();

        let _iter = stmt.query_map([], |row| {
            let is_dir_int: i32 = row.get(6)?;
            res.push(FileEntity {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                file_name: row.get(2)?,
                file_hash: row.get(3)?,
                file_size: row.get(4)?,
                file_type: row.get(5)?,
                is_dir: if is_dir_int == 1 { true } else { false },
            });
            Ok(1)
        })?;
        Ok(res)
    }

    pub fn delete_file(&self, file_id: u64) -> Result<bool> {
        let arc_conn = crate::get_connection()?;
        arc_conn.execute(
            format!("UPDATE files SET status = 0 WHERE id = {}", file_id).as_str(),
            (),
        )?;
        Ok(true)
    }

    pub fn recovery_file(&self, file_id: u64) -> Result<bool> {
        let arc_conn = crate::get_connection()?;
        arc_conn.execute(
            format!("UPDATE files SET status = 1 WHERE id = {}", file_id).as_str(),
            (),
        )?;
        Ok(true)
    }

    pub fn create_file(&self, file: &FileEntity) -> Result<u64> {
        self._create_file(file)
    }

    pub fn update_file(&self, file: &FileEntity) -> Result<u64> {
        self._update_file(file)
    }

    pub fn add_content(&self, bytes: &Vec<u8>) -> Result<String> {
        //
        ipfs_add_content(bytes.clone()).map_err(|_e| Error::IpfsConnectFailed)
    }

    pub fn get_content(&self, cid: &String) -> Result<Vec<u8>> {
        let handle = Handle::current();
        let cid_str = cid.clone();
        let handle_std = std::thread::spawn(move || {
            handle.block_on(async move { ipfs_get_content(&cid_str).await })
        });
        match handle_std.join() {
            Ok(res) => Ok(res?),
            Err(_err) => Err(Error::IpfsConnectFailed),
        }
    }

    fn _create_file(&self, file: &FileEntity) -> Result<u64> {
        let arc_conn = crate::get_connection()?;
        let is_dir_int = if file.is_dir { 1 } else { 0 };
        arc_conn.execute(
            "INSERT INTO files (parent_id, file_name, file_hash, file_size, file_type, is_dir) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&file.parent_id, &file.file_name, &file.file_hash, &file.file_size, &file.file_type, is_dir_int),
        )?;
        Ok(1)
    }

    fn _update_file(&self, file: &FileEntity) -> Result<u64> {
        //
        if file.id <= 0 {
            return self._create_file(file);
        }
        let is_dir_int = if file.is_dir { 1 } else { 0 };
        let arc_conn = crate::get_connection()?;

        arc_conn.execute(
            "UPDATE files SET parent_id = ?1, file_name = ?2, file_hash = ?3, file_size = ?4, file_type = ?5, is_dir = ?6 WHERE id = ?7",
            (&file.parent_id, &file.file_name, &file.file_hash, &file.file_size, &file.file_type, is_dir_int, file.id),
        )?;
        Ok(1)
    }
}

impl Handler for StorageServiceImpl {
    fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.storage" {
            if method_name == "list_files" {
                //
                let request = ListFilesRequest::decode(Bytes::from(message))?;
                return response(
                    self.list_files(request.parent_id)
                        .map(|r| ListFilesResponse { files: r }),
                );
            } else if method_name == "mkdir" {
                //
                let request = MkdirRequest::decode(Bytes::from(message))?;

                return response(
                    self.mkdir(request.parent_id, &request.file_name.clone())
                        .map(|r| MkdirResponse { result: r }),
                );
            } else if method_name == "list_deleted_files" {
                //
                return response(
                    self.list_deleted_files()
                        .map(|r| ListFilesResponse { files: r }),
                );
            } else if method_name == "create_file" {
                //
                let request = FileEntity::decode(Bytes::from(message))?;

                return response(self.create_file(&request).map(|r| U64Message { data: r }));
            } else if method_name == "delete_file" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.delete_file(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "recovery_file" {
                //
                let request = U64Message::decode(Bytes::from(message))?;

                return response(
                    self.recovery_file(request.data)
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_file" {
                //
                let request = FileEntity::decode(Bytes::from(message))?;

                return response(self.update_file(&request).map(|r| U64Message { data: r }));
            } else if method_name == "add_content" {
                //
                let request = BytesMessage::decode(Bytes::from(message))?;

                return response(
                    self.add_content(&request.data)
                        .map(|r| StringMessage { data: r }),
                );
            } else if method_name == "get_content" {
                //
                let request = StringMessage::decode(Bytes::from(message))?;

                return response(
                    self.get_content(&request.data)
                        .map(|r| BytesMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
