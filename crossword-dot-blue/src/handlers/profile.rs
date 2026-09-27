use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{listing::{self, Filter}, user},
    views::{self, viewer::Viewer},
};

pub async fn show(
    State(state): State<AppState>,
    viewer: Viewer,
    Path(username): Path<String>,
    Query(filter): Query<Filter>,
) -> AppResult {
    let profile = user::find_by_username(&state.db, &username).await?.ok_or(AppError::NotFound)?;
    if profile.username != username {
        return Ok(Redirect::permanent(&profile.path()).into_response());
    }

    let all = listing::by_author(&state.db, &profile.id).await?;
    let regions = listing::regions(&all);
    let entries = filter.apply(all);

    Ok(views::profile::page(&viewer, &profile, &filter, &regions, &entries).into_response())
}
