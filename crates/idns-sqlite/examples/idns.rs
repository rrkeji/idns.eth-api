use anyhow::Result;
use idns_eth_core::account::IdnsToken;
use idns_eth_sqlite::Connection;

use std::{thread, time};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let token = IdnsToken {
        public_key: String::from(idns_eth_core::account::ALICE_PUBLIC_KEY),
        application_key: String::from("IDNS.ETH"),
        token: String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwdWJsaWNfa2V5IjoiN2EwYjljOTJiNjUyMTgyMDQ0MTZkMzM1YzdiODVlZjlkNDdkYTFiYTkyYmJiMmIzYTIzMjI0YzZjZDM4Y2U1NCIsImFwcGxpY2F0aW9uX2tleSI6IklETlMuRVRIIiwiZXhwIjoxNjU5NTQxNDc3NDI3fQ.uqyybUHOvg5_weDd41KwSknqTYRYiK38pRF3MtYHnbk"),
    };
    let conn = Connection::open(&token)?;

    // conn.execute("INSERT INTO idns_rows_version(table_name, offset, size, cid) VALUES (?1, ?2, ?3, ?4) ON CONFLICT (table_name, offset, size) DO UPDATE SET cid= ?5;",
    // (&String::from("sssss"), 0, 1000, "QmU6y9P8GX4JBooP3Z822tBy7U7FQR3dANtf3HJgtcnFow","QmU6y9P8GX4JBooP3Z822tBy7U7FQR3dANtf3HJgtcnFow"))?;

    conn.execute(
        "
        CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  TEXT NOT NULL,
            _cid  TEXT DEFAULT '',
            _cn INTEGER DEFAULT 0
        );",
        (), // empty list of parameters.
    );

    // let ten_millis = time::Duration::from_millis(1000000);
    // thread::sleep(ten_millis);
    
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: "3333333".to_string()
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }

    let ten_millis = time::Duration::from_millis(1000000);
    thread::sleep(ten_millis);
    Ok(())
}
