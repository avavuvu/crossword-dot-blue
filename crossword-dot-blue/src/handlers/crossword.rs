use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use boutique::{AppError, AppResult};
use serde::Deserialize;

use crate::{
    AppState,
    models::puzzle,
    player::{Player, SetPlayer},
    views::{self, viewer::Viewer},
};

#[derive(Deserialize)]
pub struct ShareQuery {
    pub share: Option<String>,
}

pub async fn show(
    State(state): State<AppState>,
    viewer: Viewer,
    player: Player,
    Path(slug): Path<String>,
    Query(query): Query<ShareQuery>,
) -> AppResult {
    let key = puzzle::key_from_slug(&slug).ok_or(AppError::NotFound)?;
    let (model, author) = puzzle::load_playable(&state.db, key, viewer.user_id.as_deref(), query.share.as_deref()).await?;

    let canonical = model.slug(&author.username);
    if slug != canonical {
        let target = match &query.share {
            Some(token) => format!("/crossword/{canonical}?share={token}"),
            None => format!("/crossword/{canonical}"),
        };
        return Ok(Redirect::permanent(&target).into_response());
    }

    let parsed = model.puzzle()?;
    let page = views::crossword::page(&model, &author, &parsed, &viewer);
    Ok((SetPlayer(player.cookie()), page).into_response())
}
