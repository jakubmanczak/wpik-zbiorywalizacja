use std::error::Error;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::{
    api::{css, hellaur},
    database::db_check,
    html::{
        controls::{containers::controls_containers, controls},
        stats::stats,
    },
};

mod api;
mod crypto;
mod database;
mod html;
mod users;

const DEFAULT_PORT: u16 = 2025;

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
        .route("/panel/pojemniki", get(controls_containers))
        .route("/login", post(api::login_redir))
        .route("/logout", post(api::logout_redir))
        .route("/live", get(hellaur))
        .route("/styles.css", get(css))
        .route("/api/me", get(api::me));
    let l = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("listening on {}", l.local_addr()?);

    axum::serve(l, r).await?;
    Ok(())
}
