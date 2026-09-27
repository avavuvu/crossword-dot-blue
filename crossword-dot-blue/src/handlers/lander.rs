mod buzzwords;

use axum::{extract::State, response::IntoResponse};
use boutique::AppResult;

use crate::{AppState, models::listing, views::{self, viewer::Viewer}};


pub async fn show(
    State(state): State<AppState>,
    viewer: Viewer,
) -> AppResult {
    let buzzwords = buzzwords::pick(18);
    let featured = listing::featured(&state.db).await?;
    let public = listing::public(&state.db).await?;

    Ok(views::lander::page(&viewer, &buzzwords, &featured, &public).into_response())
}
