use rusqlite::Connection;

const SCHEMA: &str = include_str!("./schema.sql");

pub fn open_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(std::env::var("DB_PATH").unwrap_or(String::from("./db.db")))?;
    Ok(conn)
}

pub fn db_check() -> Result<(), rusqlite::Error> {
    let conn = open_db()?;
    if conn.table_exists(None, "logs")? {
        println!("yippee good database!")
    } else {
        println!("applying ts");
        conn.execute_batch(SCHEMA)?;
        println!("applied schema to fresh database yayy!");
    }

    Ok(())
}
