use axum::http::{HeaderMap, StatusCode};
use maud::{DOCTYPE, Markup, html};

use crate::{database::open_db, users::User};

fn head(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            link rel="stylesheet" href="/styles.css";
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

pub async fn controls(headers: HeaderMap) -> Result<Markup, (StatusCode, String)> {
    let conn =
        open_db().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "couldnt open db".into()))?;
    let user = User::authenticate(&headers, &conn).map_err(|e| (e.status_code(), e.to_string()))?;
    Ok(html! {
        (head("Zbiorywalizacja WPiK"))
        body.bg-neutral-900.text-neutral-300.min-h-screen.w-full;
        .font-serif.flex.justify-between.max-w-3xl.mx-auto.p-4 {
            p { "Zbiorywalizacja WPiK" }
            p { "Panel kontrolny" }
        }
        @if let Some(u) = user {
            .mx-auto.max-w-3xl.p-4 {
                .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                    p.font-serif.mb-4.text-center.text-xl { "Witaj, " (u.handle) "!" }
                }
            }
        }
        @else {
            .mx-auto.max-w-3xl.p-4 {
                .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                    p.font-serif.mb-4.text-center.text-xl { "Panel niedostępny bez uwierzytelnienia." }
                    .flex.gap-2.flex-wrap {
                        input.px-2.border.border-neutral-600.rounded placeholder="Login" {}
                        input.px-2.border.border-neutral-600.rounded type="password" placeholder="Hasło" {}
                        button.px-2.border.border-neutral-600.rounded { "Zaloguj się" }
                    }
                }
            }
        }
    })
}
