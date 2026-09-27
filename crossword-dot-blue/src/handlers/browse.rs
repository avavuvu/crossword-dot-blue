use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{category::Category, listing::{self, Filter}},
    views::{self, viewer::Viewer},
};

pub async fn index(
    State(state): State<AppState>,
    viewer: Viewer,
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

    Ok(views::browse::page(&viewer, category, &filter, &regions, &entries).into_response())
}
