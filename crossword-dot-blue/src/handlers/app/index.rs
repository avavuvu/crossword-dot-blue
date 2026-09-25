use axum::{extract::State, response::IntoResponse};
use boutique::AuthenticatedUser;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{AppState, error::AppResult, models::{puzzle, user}, views};

pub async fn show(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
) -> AppResult {
    let puzzles = puzzle::Entity::find()
        .filter(puzzle::Column::AuthorId.eq(&user.id))
        .order_by_desc(puzzle::Column::UpdatedAt)
        .all(&state.db)
        .await?;

    Ok(views::app::index::page(&user, puzzles).into_response())
}
