use std::error::Error;

use axum::{
    Json, Router, debug_handler,
    extract::Form,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::{
    database::{db_check, open_db},
    html::{controls, stats},
    users::{User, auth::COOKIE_NAME},
};

mod crypto;
mod database;
mod html;
mod users;

const DEFAULT_PORT: u16 = 2025;
const CSS: &str = include_str!("../web/styles.css");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let port = match std::env::var("PORT") {
        Ok(p) => p.parse::<u16>()?,
        Err(e) => match e {
            std::env::VarError::NotPresent => DEFAULT_PORT,
            _ => return Err(e)?,
        },
    };

    db_check()?;
    let r = Router::new()
        .route("/", get(stats))
        .route("/panel", get(controls))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/live", get(hellaur))
        .route("/styles.css", get(css))
        .route("/api/me", get(me));
    let l = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("listening on {}", l.local_addr()?);

    axum::serve(l, r).await?;
    Ok(())
}

async fn hellaur() -> Response {
    (StatusCode::OK, "Hello! :D").into_response()
}

async fn css() -> Response {
    ([(header::CONTENT_TYPE, "text/css")], CSS).into_response()
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(Form(form): Form<LoginForm>) -> Result<Response, (StatusCode, String)> {
    use crate::users::pwd::verify_password;
    use crate::users::sessions::Session;

    let conn =
        open_db().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "couldn't open db".into()))?;
    let user_result = conn
        .prepare("SELECT id, passhash FROM users WHERE handle = ?1")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .query_row([&form.username], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        });
    let (id_str, passhash) = match user_result {
        Ok(result) => result,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let password_valid = verify_password(&form.password, &passhash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if !password_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let user_id = uuid::Uuid::parse_str(&id_str)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = Session::create(&user_id, &conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let cookie = format!(
        "wpikzbiorauth={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        token,
        60 * 60 * 24 * 30 // 30 days in seconds
    );
    Ok(([(header::SET_COOKIE, cookie)], Redirect::to("/panel")).into_response())
}

async fn logout() -> Response {
    // Setting Max-Age 0 clears the cookie
    let cookie = format!("{COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0");

    ([(header::SET_COOKIE, cookie)], Redirect::to("/panel")).into_response()
}

#[debug_handler]
async fn me(headers: HeaderMap) -> Result<Response, (StatusCode, String)> {
    let conn =
        open_db().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "couldnt open db".into()))?;
    let user = User::authenticate(&headers, &conn)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    if let Some(u) = user {
        Ok(Json(u).into_response())
    } else {
        Ok(Json(()).into_response())
    }
}
