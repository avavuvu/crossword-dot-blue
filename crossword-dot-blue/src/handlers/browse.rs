use axum::{
    Extension,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use boutique::UserContext;

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{listing::{self, Filter}, puzzle::Category},
    views::{self, viewer::Viewer},
};

pub async fn index(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    category: Option<Path<String>>,
    Query(filter): Query<Filter>,
) -> AppResult {
    let category = match category {
        Some(Path(slug)) => Some(Category::parse(&slug).ok_or(AppError::NotFound)?),
        None => None,
    };
    let filter = match category {
        Some(category) => filter.with_category(category),
        None => filter,
    };

    let all = listing::public(&state.db).await?;
    let regions = listing::regions(&all);
    let entries = filter.apply(all);

    Ok(views::browse::page(&Viewer::from(&ctx), category, &filter, &regions, &entries).into_response())
}
