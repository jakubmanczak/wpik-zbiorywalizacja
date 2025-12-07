use rusqlite::{Connection, OptionalExtension};
use std::error::Error;
use uuid::Uuid;

use crate::{crypto::generate_short_token, users::pwd::hash_password};

const SCHEMA: &str = include_str!("./schema.sql");

pub fn open_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(std::env::var("DB_PATH").unwrap_or(String::from("./db.db")))?;
    Ok(conn)
}

pub fn db_check() -> Result<(), Box<dyn Error>> {
    let conn = open_db()?;
    if conn.table_exists(None, "logs")? {
        println!("yippee good database!")
    } else {
        println!("applying ts");
        conn.execute_batch(SCHEMA)?;
        println!("applied schema to fresh database yayy!");
    }

    if conn
        .prepare("SELECT * FROM users WHERE id = ?1")?
        .query_one([Uuid::max().to_string()], |_| Ok(()))
        .optional()?
        .is_none()
    {
        let pw = generate_short_token();
        let hash = hash_password(&pw)?;
        conn.prepare("INSERT INTO users VALUES (?1, ?2, ?3)")?
            .insert([Uuid::max().to_string(), "admin".to_owned(), hash])?;
        println!("default infradmin account was made.\nhandle: admin\npasswd: {pw}\n");
        println!("please change the password for increased safety.");
    }

    match conn
        .prepare("SELECT * FROM config WHERE id_zero = 0")?
        .query_one([], |_| Ok(()))
        .optional()?
    {
        Some(_) => (),
        None => {
            conn.prepare("INSERT INTO config DEFAULT VALUES")?
                .insert([])
                .unwrap();
        }
    };

    Ok(())
}
