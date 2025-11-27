use std::error::Error;

use axum::{
    Json, Router, debug_handler,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use tokio::net::TcpListener;

use crate::{
    database::{db_check, open_db},
    html::{controls, stats},
    users::User,
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

async fn setlogin() -> Response {
    ().into_response()
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
