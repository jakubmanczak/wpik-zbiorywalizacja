use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use maud::{Markup, html};

use crate::{
    database::open_db,
    html::{controls::controls_user_witaj, head},
    users::User,
};

pub async fn controls_containers(headers: HeaderMap) -> Response {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't open database").into_response();
        }
    };
    let user = match User::authenticate(&headers, &conn) {
        Ok(Some(u)) => u,
        Ok(None) => return Redirect::to("/panel").into_response(),
        Err(e) => return (e.status_code(), e.msg().to_string()).into_response(),
    };
    let containers = match conn
        .prepare("SELECT id, name FROM containers")
        .unwrap()
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .and_then(|rows| rows.collect::<Result<Vec<_>, _>>())
    {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read container data",
            )
                .into_response();
        }
    };

    (html! {
        (head("Zbiorywalizacja WPiK"))
        body.bg-neutral-900.text-neutral-300.min-h-screen.w-full;
        .font-serif.flex.justify-between.max-w-3xl.mx-auto.p-4 {
            a href="/" { p { "Zbiorywalizacja WPiK" } }
            a href="/panel" { p { "Panel kontrolny" } }
        }
        (controls_user_witaj(user))
        (containers_list(containers))
        (new_container())
    })
    .into_response()
}

fn containers_list(containers: Vec<(String, String)>) -> Markup {
    html! {
        .mx-auto.max-w-3xl.p-4 {
            p.font-serif.text-xl.ml-1 {"Pojemniki"}
            .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                @for container in containers {
                    p {(container.1) " (" (container.0) ")" }
                }
            }
        }
    }
}

fn new_container() -> Markup {
    html! {
        .mx-auto.max-w-3xl.p-4 {
            p.font-serif.text-xl.ml-1 { "Nowy pojemnik" }
            .w-full.p-4.bg-neutral-800.text-neutral-200.rounded.border.border-neutral-600 {
                form .flex.flex-col.gap-1 {
                    label for="contname" .mr-4{"Nazwa pojemnika"}
                    input name="contname" .mb-3.py-1.px-2.border.border-neutral-600.rounded.bg-neutral-900;
                    button type="submit" .p-1.px-2.border.border-neutral-600.rounded.ml-auto {"Utwórz pojemnik"}
                    // label for="contrbank" .mr-4 { "Pojemnik" }
                    // select name="contrbank" id="contrbank" .mb-3.p-2.border.border-neutral-600.rounded.bg-neutral-900 {
                    //     @for (id, name) in containers {
                    //         option value=(id) { (name) }
                    //     }
                    // }
                    // label for="contramt" .mr-4{"Wielkość datku " span.text-neutral-500{"(w zł)"} }
                    // input name="contramt" type="number" step="0.01" min="0" required
                    //     value=(format!("{:.2}", default_contramt as f64 / 100.0))
                    //     .mb-3.py-1.px-2.border.border-neutral-600.rounded.bg-neutral-900;
                    // label for "contrnote" .mr-4{"Notatka do datku " span.text-neutral-500{"(opcjonalnie)"}}
                    // input name="contrnote" type="text" .mb-3.py-1.px-2.border.border-neutral-600.rounded.bg-neutral-900;
                    // button type="submit" .p-1.px-2.border.border-neutral-600.rounded.ml-auto { "Odnotuj datek" }
                }
            }
        }
    }
}
