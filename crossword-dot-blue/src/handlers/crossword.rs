use axum::{
    Extension,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use boutique::UserContext;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Deserialize;

use crate::{AppState, error::{AppError, AppResult}, models::{puzzle, user}, views::{self, state::ViewState}};

#[derive(Deserialize)]
pub struct ShowQuery {
    share: Option<String>,
}

pub async fn show(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    Path(slug): Path<String>,
    Query(query): Query<ShowQuery>,
) -> AppResult {
    let key = puzzle::key_from_slug(&slug).ok_or(AppError::NotFound)?;

    let model = puzzle::Entity::find()
        .filter(puzzle::Column::Id.starts_with(key))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let is_author = ctx.user_id.as_deref() == Some(model.author_id.as_str());
    let shared = model.accepts_share(query.share.as_deref());
    if !model.is_public && !is_author && !shared {
        return Err(AppError::NotFound);
    }

    let author = model
        .find_related(user::Entity)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let canonical = model.slug(&author.username);
    if slug != canonical {
        let target = match &query.share {
            Some(token) => format!("/crossword/{canonical}?share={token}"),
            None => format!("/crossword/{canonical}"),
        };
        return Ok(Redirect::permanent(&target).into_response());
    }

    let content = model.content()?;
    Ok(views::crossword::show(&model, &author, &content, &ViewState::from(&ctx)).into_response())
}
