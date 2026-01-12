use maud::{DOCTYPE, Markup, html};

pub mod controls;
pub mod stats;

pub fn head(title: &str) -> Markup {
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

pub const JS_CLEAN_QUERY: &str = r#"
if (window.location.search) {
    history.replaceState({}, '', window.location.pathname);
}
"#;
pub const SVG_PACKAGE_OPEN: &str = include_str!("../lucideicons/package-open.svg");
pub const SVG_SETTINGS: &str = include_str!("../lucideicons/settings.svg");
