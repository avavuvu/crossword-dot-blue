use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use boutique::{AppError, AppResult};

use crate::{
    AppState,
    handlers::crossword::ShareQuery,
    models::{progress::{self, Owner, Saved, Stored}, puzzle},
    player::{Player, SetPlayer},
};

async fn playable(state: &AppState, player: &Player, key: &str, share: Option<&str>) -> AppResult<puzzle::Model> {
    let (model, _) = puzzle::load_playable(&state.db, key, player.user_id.as_deref(), share).await?;
    Ok(model)
}

fn owner(player: &Player) -> Owner<'_> {
    Owner { player_id: &player.id, user_id: player.user_id.as_deref() }
}

pub async fn show(
    State(state): State<AppState>,
    player: Player,
    Path(key): Path<String>,
    Query(query): Query<ShareQuery>,
) -> AppResult {
    let model = playable(&state, &player, &key, query.share.as_deref()).await?;

    let body = match progress::find_for(&state.db, &owner(&player), &model.id).await? {
        Some(row) => Json(row.stored(&model.key)).into_response(),
        None => StatusCode::NO_CONTENT.into_response(),
    };

    Ok((SetPlayer(player.cookie()), body).into_response())
}

pub async fn save(
    State(state): State<AppState>,
    player: Player,
    Path(key): Path<String>,
    Query(query): Query<ShareQuery>,
    Json(stored): Json<Stored>,
) -> AppResult {
    let model = playable(&state, &player, &key, query.share.as_deref()).await?;
    if !stored.is_valid_for(&model.key) {
        return Err(AppError::BadRequest);
    }

    let body = match progress::save(&state.db, &owner(&player), &model.id, stored).await? {
        Saved::Ok { saved_at, solved_at } => {
            Json(serde_json::json!({ "savedAt": saved_at, "solvedAt": solved_at })).into_response()
        }
        Saved::Stale(newest) => (StatusCode::CONFLICT, Json(newest.stored(&model.key))).into_response(),
    };

    Ok((SetPlayer(player.cookie()), body).into_response())
}

pub async fn delete(
    State(state): State<AppState>,
    player: Player,
    Path(key): Path<String>,
    Query(query): Query<ShareQuery>,
) -> AppResult {
    let model = playable(&state, &player, &key, query.share.as_deref()).await?;
    progress::delete_for(&state.db, &owner(&player), &model.id).await?;

    Ok((SetPlayer(player.cookie()), StatusCode::NO_CONTENT).into_response())
}
