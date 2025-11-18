use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand08::rngs::OsRng;
use rusqlite::{Connection, OptionalExtension};
use std::error::Error;
use uuid::Uuid;

use crate::crypto::generate_short_token;

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
        conn.prepare("INSERT INTO users VALUES (?1, ?2, ?3)")?
            .insert([Uuid::max().to_string(), "admin".to_owned(), {
                let argon = Argon2::default();
                let salt = SaltString::generate(&mut OsRng);
                argon
                    .hash_password(pw.as_bytes(), &salt)
                    .unwrap()
                    .to_string()
            }])?;
        println!("default infradmin account was made. credentials: admin {pw}");
    }

    Ok(())
}
