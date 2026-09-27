use axum::response::IntoResponse;

use crate::views::{self, text::TextPage, viewer::Viewer};

macro_rules! text_page {
    ($name:ident, $slug:literal, $title:literal) => {
        pub async fn $name(viewer: Viewer) -> impl IntoResponse {
            let text = TextPage {
                title: $title,
                html: include_str!(concat!(env!("OUT_DIR"), "/content/", $slug, ".html")),
            };
            views::text::page(&text, &viewer)
        }
    };
}

text_page!(about, "about", "About");
text_page!(privacy, "privacy", "Privacy policy");
