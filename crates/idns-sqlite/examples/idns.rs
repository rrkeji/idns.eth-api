use idns_eth_sqlite::{Connection, Result};

use idns_eth_core::account::IdnsToken;

use std::{thread, time};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let database_filename = "data.db";

    let token = IdnsToken {
        public_key: Some(String::from("")),
        application_key: None,
        token: None,
    };
    let conn = Connection::open(database_filename, &token)?;

    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
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
        println!("Found person {:?}", person.unwrap());
    }

    let ten_millis = time::Duration::from_millis(1000000);
    thread::sleep(ten_millis);
    Ok(())
}
