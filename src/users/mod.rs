use std::str::FromStr;

use rusqlite::Connection;
use uuid::Uuid;

pub mod pwd;
pub mod sessions;

pub struct User {
    id: Uuid,
    handle: String,
}

#[derive(thiserror::Error, Debug)]
pub enum UserStructError {
    #[error("Failed to execute SQL: {0}")]
    UserSqlError(#[from] rusqlite::Error),
    #[error("Non-UUID User PK found in DB")]
    NonUuidPrimaryKey,
}

impl User {
    pub fn is_infradmin(&self) -> bool {
        self.id.is_max()
    }
    pub fn get_by_uuid(uuid: &Uuid, conn: &Connection) -> Result<User, UserStructError> {
        let pk = uuid.to_string();
        let (id, handle) = conn
            .prepare("SELECT * FROM users WHERE id = ?1")?
            .query_row([&pk], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
        Ok(User {
            id: Uuid::from_str(&id).map_err(|_| UserStructError::NonUuidPrimaryKey)?,
            handle,
        })
    }
}
