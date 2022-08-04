use anyhow::Result;
use idns_eth_core::account::IdnsToken;
use idns_eth_sqlite::Connection;
use rusqlite::OpenFlags;
use std::path::Path;
use std::{thread, time};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::debug!("download start........");
    let token = IdnsToken {
        public_key: String::from(idns_eth_core::account::ALICE_PUBLIC_KEY),
        application_key: String::from("IDNS.ETH"),
        token: String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwdWJsaWNfa2V5IjoiN2EwYjljOTJiNjUyMTgyMDQ0MTZkMzM1YzdiODVlZjlkNDdkYTFiYTkyYmJiMmIzYTIzMjI0YzZjZDM4Y2U1NCIsImFwcGxpY2F0aW9uX2tleSI6IklETlMuRVRIIiwiZXhwIjoxNjU5NTQxNDc3NDI3fQ.uqyybUHOvg5_weDd41KwSknqTYRYiK38pRF3MtYHnbk"),
    };
    let path_str = idns_eth_sqlite::utils::get_database_path(&token.application_key)?;
    let path = Path::new(path_str.as_str());

    let flags = OpenFlags::default();
    let conn = rusqlite::Connection::open_with_flags(path, flags).unwrap();

    // let conn = Connection::open(database_filename, &token)?;
    idns_eth_sqlite::sync::DataBaseSync::download(&conn, &token)
        .await
        .unwrap();

    // let ten_millis = time::Duration::from_millis(1000000);
    // thread::sleep(ten_millis);
    Ok(())
}
