use axum::{extract::{Path, Query, State}, response::IntoResponse};

use crate::{AppState, error::{AppError, AppResult}, handlers::crossword::ShareQuery, models::puzzle, player::{Player, SetPlayer}, views::{self, viewer::Viewer}};

pub async fn show(
    State(state): State<AppState>,
    viewer: Viewer,
    player: Player,
    Path(slug): Path<String>,
    Query(query): Query<ShareQuery>,
) -> AppResult {
    let key = puzzle::key_from_slug(&slug).ok_or(AppError::NotFound)?;
    let (model, _) = puzzle::load_playable(&state.db, key, viewer.user_id.as_deref(), query.share.as_deref()).await?;

    let parsed = model.puzzle()?;

    let page = views::embed::page(&model, &parsed);
    Ok((SetPlayer(player.cookie()), page).into_response())
}
