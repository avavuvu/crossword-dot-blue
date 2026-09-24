mod buzzwords;

use axum::{Extension, extract::State, response::IntoResponse};
use boutique::UserContext;

use crate::{AppState, error::AppResult, models::listing, views::{self, state::ViewState}};

const BUZZWORD_COUNT: usize = 18;

pub async fn lander(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
) -> AppResult {
    let buzzwords = buzzwords::pick(BUZZWORD_COUNT);
    let featured = listing::featured(&state.db).await?;
    let public = listing::public(&state.db).await?;

    Ok(views::lander::index(&ViewState::from(&ctx), &buzzwords, &featured, &public).into_response())
}
