use axum::extract::Query;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;

use crate::users::auth::COOKIE_CLEAR;
use crate::{database::open_db, users::User};

fn head(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            link rel="stylesheet" href="/styles.css";
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { (title) }
        }
    }
}

pub async fn stats() -> Markup {
    html! {
        (head("Zbiorywalizacja WPiK"))
        body.bg-neutral-900.text-neutral-300.min-h-screen.w-full {
            p.text-center.text-2xl.font-serif.pt-16 { "Zbiorywalizacja WPiK" }
        }
    }
}

pub const JS_CLEAN_QUERY: &str = r#"
if (window.location.search) {
    history.replaceState({}, '', window.location.pathname);
}
"#;

#[derive(Deserialize)]
pub struct LoginErrorQuery {
    pub error: Option<String>,
}
pub async fn controls(headers: HeaderMap, Query(query): Query<LoginErrorQuery>) -> Response {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't open database").into_response();
        }
    };
    let (user, error_msg) = match User::authenticate(&headers, &conn) {
        Ok(user) => (user, query.error),
        Err(e) => (None, Some(e.msg().to_string())),
    };
    let defcontramt = conn
        .prepare("SELECT default_contribution_amount FROM config WHERE id_zero = 0")
        .unwrap()
        .query_one([], |r| Ok(r.get::<_, u32>(0).unwrap()))
        .unwrap();

    (
        [if error_msg.is_some() {
            (header::SET_COOKIE.as_str(), COOKIE_CLEAR)
        } else {
            ("auth", "good")
        }],
        html! {
            (head("Zbiorywalizacja WPiK"))
            body.bg-neutral-900.text-neutral-300.min-h-screen.w-full;
            .font-serif.flex.justify-between.max-w-3xl.mx-auto.p-4 {
                a href="/" { p { "Zbiorywalizacja WPiK" } }
                p { "Panel kontrolny" }
            }
            @if let Some(u) = user {
                (controls_user_witaj(u))
                (controls_new_contributions(defcontramt))
            }
            @else {
                (controls_user_login(error_msg))
            }
        },
    )
        .into_response()
}

const OPTIONS: &[&str] = &["Kognitywistyka", "Psychologia"];
fn controls_new_contributions(default_contramt: u32) -> Markup {
    html! {
        .mx-auto.max-w-3xl.p-4 {
            p.font-serif.text-xl.ml-1 { "Nowy datek" }
            .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                label for="contrcontainer" class="mr-4" { "Pojemnik" }
                select name="contrcontainer" id="contrcontainer" {
                    @for c in OPTIONS {
                        option value=(c) { (c) }
                    }
                }
                br;
                label for="contramt" class="mr-4" { "Amount" }
                input name="contramt" type="number" step="0.01" min="0" required value=(
                    format!("{}.{}", default_contramt/100, default_contramt%100)
                );
            }
        }
    }
}

fn controls_user_witaj(u: User) -> Markup {
    html! {
        .mx-auto.max-w-3xl.p-4 {
            .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                .flex.flex-row.justify-between {
                    p.font-serif.text-center.text-xl { "Witaj, " (u.handle) "!" }
                    form.flex.justify-center method="post" action="/logout" {
                        button.px-2.border.border-neutral-600.rounded.hover:bg-neutral-700.cursor-pointer
                            type="submit" { "Wyloguj się" }
                    }
                }
            }
        }
    }
}

fn controls_user_login(error_msg: Option<String>) -> Markup {
    html! {
        .mx-auto.max-w-3xl.p-4 {
            .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                p.font-serif.mb-4.text-center.text-xl { "Panel niedostępny bez uwierzytelnienia." }
                @if let Some(error_msg) = error_msg {
                    .mb-4.p-3.bg-red-900.bg-opacity-50.border.border-red-700.rounded.text-red-200 {
                        p { (error_msg) }
                    }
                    script { (JS_CLEAN_QUERY) }
                }
                form.flex.gap-2.flex-wrap method="post" action="/login" {
                    input.px-2.border.border-neutral-600.rounded.bg-neutral-900
                        name="username" placeholder="Login" required {}
                    input.px-2.border.border-neutral-600.rounded.bg-neutral-900
                        type="password" name="password" placeholder="Hasło" required {}
                    button.px-2.border.border-neutral-600.rounded.hover:bg-neutral-700.cursor-pointer
                        type="submit" { "Zaloguj się" }
                }
            }
        }
    }
}
