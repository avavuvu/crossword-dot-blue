use axum::Extension;
use boutique::UserContext;
use maud::Markup;

use crate::views::{self, state::ViewState};

pub struct TextPage {
    pub title: &'static str,
    pub html: &'static str,
}

macro_rules! text_page {
    ($slug:literal, $title:literal) => {
        TextPage {
            title: $title,
            html: include_str!(concat!(env!("OUT_DIR"), "/content/", $slug, ".html")),
        }
    };
}

pub async fn about(Extension(ctx): Extension<UserContext>) -> Markup {
    views::text::show(&text_page!("about", "About"), &ViewState::from(&ctx))
}

pub async fn privacy(Extension(ctx): Extension<UserContext>) -> Markup {
    views::text::show(&text_page!("privacy", "Privacy policy"), &ViewState::from(&ctx))
}
