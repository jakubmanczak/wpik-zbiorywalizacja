use rusqlite::Connection;

pub fn open_db() -> Result<Connection, rusqlite::Error> {
    Connection::open(std::env::var("DB_PATH").unwrap_or(String::from("./db.db")))
}

pub fn db_check() -> Result<(), rusqlite::Error> {
    let conn = open_db()?;

    Ok(())
}
