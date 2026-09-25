use axum::{Extension, extract::MatchedPath, response::IntoResponse};
use boutique::UserContext;

use crate::{
    error::{AppError, AppResult},
    views::{self, text::TextPage, viewer::Viewer},
};

macro_rules! text_page {
    ($slug:literal, $title:literal) => {
        TextPage {
            title: $title,
            html: include_str!(concat!(env!("OUT_DIR"), "/content/", $slug, ".html")),
        }
    };
}

pub async fn show(path: MatchedPath, Extension(ctx): Extension<UserContext>) -> AppResult {
    let text = match path.as_str() {
        "/about" => text_page!("about", "About"),
        "/privacy" => text_page!("privacy", "Privacy policy"),
        _ => return Err(AppError::NotFound),
    };

    Ok(views::text::page(&text, &Viewer::from(&ctx)).into_response())
}
