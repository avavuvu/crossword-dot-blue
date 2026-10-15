use axum::http::HeaderMap;
use boutique::{AppError, ErrorReport, views::back_url};
use maud::Markup;

use crate::{theme::Theme, views::{error, viewer::Viewer}};

pub async fn render(viewer: Result<Viewer, AppError>, headers: HeaderMap, report: ErrorReport) -> Markup {
    let viewer = viewer.unwrap_or_else(|_| Viewer { user_id: None, theme: Theme::from_headers(&headers) });
    error::page(&report, back_url(&headers), &viewer)
}
