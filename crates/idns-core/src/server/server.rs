use crate::idns_home_path;
use actix_files as fs;
use actix_web::{get, http::header, web, App, HttpServer, Responder};
use anyhow::{anyhow, Result};
use tokio::runtime::Handle;

pub struct Server {}

impl Server {
    ///获取url的根路径
    pub fn get_root_uri() -> Result<String> {
        Ok(String::from("http://127.0.0.1:35080"))
    }
    /// 下载应用
    pub fn download(app_cid: &String, category: &String) -> Result<()> {
        //根据分类下载到不同的文件夹

        Ok(())
    }

    ///启动服务
    pub fn start() -> Result<()> {
        tokio::spawn(
            HttpServer::new(|| {
                //静态文件路径
                let storage_path = idns_home_path().unwrap().join("webroot");
                std::fs::create_dir_all(storage_path.as_path());

                tracing::info!("server web root:{:?}.", storage_path.to_str());

                App::new()
                    .wrap(actix_web::middleware::Compress::default())
                    .wrap(
                        actix_web::middleware::DefaultHeaders::new()
                            .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")),
                    )
                    .wrap(actix_web::middleware::NormalizePath::trim())
                    .wrap(actix_web::middleware::Logger::default())
                    .service(
                        fs::Files::new("/", storage_path.to_str().unwrap()).show_files_listing(),
                    )
            })
            .bind(("0.0.0.0", 35080))?
            .run(),
        );
        Ok(())
    }
}
