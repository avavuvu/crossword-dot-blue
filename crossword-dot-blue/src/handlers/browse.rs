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
    views::{self, state::ViewState},
};

pub async fn index(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    Query(filter): Query<Filter>,
) -> AppResult {
    render(&state, &ctx, filter, None).await
}

pub async fn category(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    Path(category): Path<String>,
    Query(filter): Query<Filter>,
) -> AppResult {
    let category = Category::parse(&category).ok_or(AppError::NotFound)?;
    render(&state, &ctx, filter.with_category(category), Some(category)).await
}

async fn render(state: &AppState, ctx: &UserContext, filter: Filter, category: Option<Category>) -> AppResult {
    let all = listing::public(&state.db).await?;
    let regions = listing::regions(&all);
    let entries = filter.apply(all);

    Ok(views::browse::index(&ViewState::from(ctx), category, &filter, &regions, &entries).into_response())
}
