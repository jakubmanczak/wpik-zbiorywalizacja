use axum::http::{
    HeaderMap, StatusCode,
    header::{AUTHORIZATION, COOKIE},
};
use base64::prelude::*;
use rusqlite::Connection;

use std::str::FromStr;
use uuid::Uuid;

use crate::users::pwd::verify_password;
use crate::users::sessions::{Session, SessionStructError};
use crate::users::{User, UserStructError};

const COOKIE_NAME: &str = "wpikzbiorauth";

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Session error: {0}")]
    SessionError(#[from] SessionStructError),
    #[error("User error: {0}")]
    UserError(#[from] UserStructError),
    #[error("Invalid authorization header format")]
    InvalidFormat,
    #[error("Invalid base64 encoding")]
    InvalidBase64(#[from] base64::DecodeError),
    #[error("Invalid UTF-8 in credentials")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
}
impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        use AuthError as AE;
        match self {
            AE::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AE::SessionError(_) | AE::UserError(_) | AE::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AE::InvalidFormat | AE::InvalidUtf8(_) | AE::InvalidBase64(_) => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}

enum AuthScheme<'a> {
    Basic(&'a str),
    Bearer(&'a str),
    None,
}

impl<'a> AuthScheme<'a> {
    fn from_header(header: &'a str) -> Self {
        if let Some(credentials) = header
            .strip_prefix("Basic ")
            .or_else(|| header.strip_prefix("basic "))
        {
            AuthScheme::Basic(credentials)
        } else if let Some(token) = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
        {
            AuthScheme::Bearer(token)
        } else {
            AuthScheme::None
        }
    }
}

impl User {
    pub fn authenticate(headers: &HeaderMap, conn: &Connection) -> Result<Option<User>, AuthError> {
        let mut auth_values = Vec::new();

        for header_value in headers.get_all(AUTHORIZATION).iter() {
            if let Ok(s) = header_value.to_str() {
                auth_values.push(s.to_string());
            }
        }
        for cookie_header in headers.get_all(COOKIE).iter() {
            if let Ok(cookies) = cookie_header.to_str() {
                for cookie in cookies.split(';') {
                    let cookie = cookie.trim();
                    if let Some(value) = cookie.strip_prefix(&format!("{}=", COOKIE_NAME)) {
                        auth_values.push(format!("Bearer {}", value));
                    }
                }
            }
        }

        let mut basic_auth: Option<&str> = None;
        let mut bearer_auth: Option<&str> = None;
        for auth_header in &auth_values {
            let auth_header = auth_header.trim();
            match AuthScheme::from_header(auth_header) {
                AuthScheme::Basic(credentials) => {
                    if basic_auth.is_none() {
                        basic_auth = Some(credentials);
                        break;
                    }
                }
                AuthScheme::Bearer(token) => {
                    if bearer_auth.is_none() {
                        bearer_auth = Some(token);
                    }
                }
                AuthScheme::None => {}
            }
        }

        // Try authentication in priority order
        match (basic_auth, bearer_auth) {
            (Some(credentials), _) => authenticate_basic(credentials, conn),
            (None, Some(token)) => authenticate_bearer(token, conn),
            (None, None) => Ok(None),
        }
    }
}

fn authenticate_basic(credentials: &str, conn: &Connection) -> Result<Option<User>, AuthError> {
    let decoded = BASE64_STANDARD.decode(credentials)?;
    let credentials_str = String::from_utf8(decoded)?;

    let Some((username, password)) = credentials_str.split_once(':') else {
        return Err(AuthError::InvalidFormat);
    };

    let user_result = conn
        .prepare("SELECT id, passhash FROM users WHERE handle = ?1")?
        .query_row([username], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        });

    match user_result {
        Ok((id_str, passhash)) => {
            if verify_password(password, &passhash).map_err(|_| AuthError::InvalidCredentials)? {
                let user_id =
                    Uuid::from_str(&id_str).map_err(|_| UserStructError::NonUuidPrimaryKey)?;
                let user = User::get_by_uuid(&user_id, conn)?;
                Ok(Some(user))
            } else {
                Err(AuthError::InvalidCredentials)
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(AuthError::InvalidCredentials),
        Err(e) => Err(AuthError::DatabaseError(e)),
    }
}

fn authenticate_bearer(token: &str, conn: &Connection) -> Result<Option<User>, AuthError> {
    let session = Session::get_by_token(token, conn)?;

    if session.is_expired_or_revoked() {
        return Err(AuthError::InvalidCredentials);
    }

    let user = User::get_by_uuid(session.user_id(), conn)?;
    Ok(Some(user))
}
