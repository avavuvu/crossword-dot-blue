use axum::{
    Form,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;

use crate::{AppState, error::{AppError, AppResult}, theme::Theme, views::{self, viewer::Viewer}};

#[derive(Deserialize)]
pub struct OptionsForm {
    theme: String,
}

pub async fn show(viewer: Viewer) -> AppResult {
    Ok(views::options::page(&viewer).into_response())
}

pub async fn update(State(state): State<AppState>, headers: HeaderMap, Form(form): Form<OptionsForm>) -> AppResult {
    let theme = Theme::parse(&form.theme).ok_or(AppError::BadRequest)?;
    let cookie = theme.cookie(state.config.secure_cookies);

    let wants_json = headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|accept| accept.contains("application/json"));

    if wants_json {
        return Ok((cookie, StatusCode::NO_CONTENT).into_response());
    }

    let back = headers
        .get(header::REFERER)
        .and_then(|value| value.to_str().ok())
        .filter(|referer| referer.starts_with('/') || referer.starts_with(crate::config::get().site_url.as_str()))
        .unwrap_or("/options");

    Ok((cookie, Redirect::to(back)).into_response())
}
