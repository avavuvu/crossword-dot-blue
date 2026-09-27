use axum::{extract::State, response::IntoResponse};
use boutique::AuthenticatedUser;
use sea_orm::{ModelTrait, QueryOrder};

use crate::{AppState, error::AppResult, models::{puzzle, user}, views::{self, viewer::Viewer}};

pub async fn show(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    viewer: Viewer,
) -> AppResult {
    let puzzles = user
        .find_related(puzzle::Entity)
        .order_by_desc(puzzle::Column::UpdatedAt)
        .all(&state.db)
        .await?;

    Ok(views::app::index::page(&user, &viewer, puzzles).into_response())
}
