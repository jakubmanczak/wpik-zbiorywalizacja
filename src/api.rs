use axum::{
    Form, Json,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    database::open_db,
    users::{
        User,
        auth::{COOKIE_CLEAR, COOKIE_NAME},
    },
};

const CSS: &str = include_str!("../web/styles.css");

pub async fn hellaur() -> Response {
    (StatusCode::OK, "Hello! :D").into_response()
}

pub async fn css() -> Response {
    ([(header::CONTENT_TYPE, "text/css")], CSS).into_response()
}

pub async fn me(headers: HeaderMap) -> Result<Response, (StatusCode, String)> {
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

pub async fn logout_redir() -> Response {
    ([(header::SET_COOKIE, COOKIE_CLEAR)], Redirect::to("/panel")).into_response()
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_redir(Form(form): Form<LoginForm>) -> Response {
    use crate::users::pwd::verify_password;
    use crate::users::sessions::Session;

    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => {
            return Redirect::to("/panel?error=Błąd serwera. Skontaktuj się z webmasterem.")
                .into_response();
        }
    };

    let user_result = conn
        .prepare("SELECT id, passhash FROM users WHERE handle = ?1")
        .and_then(|mut stmt| {
            stmt.query_row([&form.username], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
        });

    let (id_str, passhash) = match user_result {
        Ok(result) => result,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Redirect::to("/panel?error=Nieprawidłowy login lub hasło.").into_response();
        }
        Err(_) => {
            return Redirect::to("/panel?error=Błąd serwera. Skontaktuj się z webmasterem.")
                .into_response();
        }
    };

    let password_valid = match verify_password(&form.password, &passhash) {
        Ok(valid) => valid,
        Err(_) => {
            return Redirect::to("/panel?error=Błąd+weryfikacji+hasła.").into_response();
        }
    };

    if !password_valid {
        return Redirect::to("/panel?error=Nieprawidłowy+login+lub+hasło.").into_response();
    }

    let user_id = match uuid::Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            return Redirect::to("/panel?error=Błąd+serwera.+Spróbuj+ponownie+później.")
                .into_response();
        }
    };

    let token = match Session::create(&user_id, &conn) {
        Ok(t) => t,
        Err(_) => {
            return Redirect::to("/panel?error=Nie+udało+się+utworzyć+sesji.").into_response();
        }
    };

    let secure = match cfg!(debug_assertions) {
        false => "; Secure",
        true => "",
    };
    let cookie = format!(
        "{COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}{}",
        60 * 60 * 24 * 30, // 30 days in seconds
        secure
    );
    ([(header::SET_COOKIE, cookie)], Redirect::to("/panel")).into_response()
}
