use std::error::Error;

use axum::{
    Router,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use tokio::net::TcpListener;

use crate::{
    database::db_check,
    html::{controls, stats},
};

mod crypto;
mod database;
mod html;

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
        .route("/styles.css", get(css));
    let l = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("Listening on {}", l.local_addr()?);

    axum::serve(l, r).await?;
    Ok(())
}

async fn hellaur() -> Response {
    (StatusCode::OK, "Hello! :D").into_response()
}

async fn css() -> Response {
    ([(header::CONTENT_TYPE, "text/css")], CSS).into_response()
}
