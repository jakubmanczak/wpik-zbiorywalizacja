use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;
use uuid::Uuid;

use crate::crypto::generate_short_token;

pub struct Session {
    id: Uuid,
    user_id: Uuid,
    expiry: DateTime<Utc>,
    last_access: DateTime<Utc>,
    revoked: bool,
    revoked_at: Option<DateTime<Utc>>,
}

#[derive(thiserror::Error, Debug)]
pub enum SessionStructError {
    #[error("Failed to execute SQL: {0}")]
    UserSqlError(#[from] rusqlite::Error),
    #[error("Non-UUID Session PK found in DB")]
    NonUuidPrimaryKey,
    #[error("Non-UUID UserId found in DB")]
    NonUuidUserId,
}

impl Session {
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn is_expired_or_revoked(&self) -> bool {
        self.expiry <= Utc::now() || self.revoked
    }

    /// Create a new session for a user. Returns the session token that should be stored in the cookie.
    pub fn create(user_id: &Uuid, conn: &Connection) -> Result<String, SessionStructError> {
        let session_id = Uuid::now_v7();
        let token = generate_short_token();
        let now = Utc::now();
        let expiry = now + Duration::days(30); // 30 day expiry

        conn.prepare(
            "INSERT INTO sessions (id, token, user_id, expiry, last_access, revoked)
             VALUES (?1, ?2, ?3, ?4, ?5, 0)",
        )?
        .execute([
            session_id.to_string(),
            token.clone(),
            user_id.to_string(),
            expiry.timestamp().to_string(),
            now.timestamp().to_string(),
        ])?;

        Ok(token)
    }
    pub fn issued(&self) -> DateTime<Utc> {
        let timestamp = self.id.get_timestamp().unwrap();
        DateTime::from_timestamp_millis(timestamp.to_unix().0 as i64).unwrap()
    }
    pub fn get_by_id(id: &Uuid, conn: &Connection) -> Result<Session, SessionStructError> {
        let pk = id.to_string();
        const QUERY: &str = "
            SELECT id, user_id, expiry, last_access, revoked, revoked_at
            FROM sessions WHERE id = ?1
        ";
        let res = conn.prepare(QUERY)?.query_one([&pk], |row| {
            Ok((
                row.get::<_, String>(0)?,      // id
                row.get::<_, String>(1)?,      // user_id
                row.get::<_, i64>(2)?,         // expiry
                row.get::<_, i64>(3)?,         // last_access
                row.get::<_, bool>(4)?,        // revoked
                row.get::<_, Option<i64>>(5)?, // revoked_at
            ))
        })?;
        Ok(Session {
            id: Uuid::from_str(&res.0).map_err(|_| SessionStructError::NonUuidPrimaryKey)?,
            user_id: Uuid::from_str(&res.1).map_err(|_| SessionStructError::NonUuidUserId)?,
            expiry: DateTime::from_timestamp(res.2, 0).unwrap(),
            last_access: DateTime::from_timestamp(res.3, 0).unwrap(),
            revoked: res.4,
            revoked_at: res.5.map(|ts| DateTime::from_timestamp(ts, 0).unwrap()),
        })
    }
    pub fn get_by_token(token: &str, conn: &Connection) -> Result<Session, SessionStructError> {
        const QUERY: &str = "
            SELECT id, user_id, expiry, last_access, revoked, revoked_at
            FROM sessions WHERE token = ?1
        ";
        let res = conn.prepare(QUERY)?.query_one([token], |row| {
            Ok((
                row.get::<_, String>(0)?,      // id
                row.get::<_, String>(1)?,      // user_id
                row.get::<_, i64>(2)?,         // expiry
                row.get::<_, i64>(3)?,         // last_access
                row.get::<_, bool>(4)?,        // revoked
                row.get::<_, Option<i64>>(5)?, // revoked_at
            ))
        })?;
        Ok(Session {
            id: Uuid::from_str(&res.0).map_err(|_| SessionStructError::NonUuidPrimaryKey)?,
            user_id: Uuid::from_str(&res.1).map_err(|_| SessionStructError::NonUuidUserId)?,
            expiry: DateTime::from_timestamp(res.2, 0).unwrap(),
            last_access: DateTime::from_timestamp(res.3, 0).unwrap(),
            revoked: res.4,
            revoked_at: res.5.map(|ts| DateTime::from_timestamp(ts, 0).unwrap()),
        })
    }
}
