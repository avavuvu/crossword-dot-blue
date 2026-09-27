pub mod clues;

use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use boutique::{AuthenticatedUser, htmx, validator::Validate};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ModelTrait};
use serde::Deserialize;

use crate::{
    AppState,
    error::{AppError, AppResult},
    handlers::form::blank_to_none,
    ids,
    models::{puzzle, region::{self, Region}, user},
    views::{self, viewer::Viewer},
};

#[derive(Deserialize, Validate)]
pub struct EditForm {
    #[validate(length(max = 200, message = "Title must be 200 characters or fewer"))]
    pub title: String,
    pub notes: String,
    pub difficulty: String,
    pub region: String,
    #[serde(default)]
    pub themed: Option<String>,
    #[serde(default)]
    pub is_cryptic: Option<String>,
    #[serde(default)]
    pub is_public: Option<String>,
}

pub(super) async fn find_puzzle(state: &AppState, user: &user::Model, key: &str) -> AppResult<(puzzle::Model, user::Model)> {
    let (model, author) = puzzle::find_with_author(&state.db, key).await?;
    if !user.is_admin && author.id != user.id {
        return Err(AppError::NotFound);
    }

    Ok((model, author))
}

pub async fn show(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    viewer: Viewer,
    Path(key): Path<String>,
) -> AppResult {
    let (model, author) = find_puzzle(&state, &user, &key).await?;
    let puzzle = model.puzzle()?;
    Ok(views::app::edit::page(&model, &author, &user, &viewer, &puzzle).into_response())
}

pub async fn update(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
    Form(form): Form<EditForm>,
) -> AppResult {
    form.validate()?;

    let difficulty = match form.difficulty.trim() {
        "" => None,
        value => match value.parse::<i16>() {
            Ok(d) if (1..=5).contains(&d) => Some(d),
            _ => return Err(AppError::field("difficulty", "Difficulty must be a number from 1 to 5")),
        },
    };

    if form.region.trim().chars().count() > region::MAX_LEN {
        return Err(AppError::field("region", format!("Region must be {} characters or fewer", region::MAX_LEN)));
    }

    let (model, author) = find_puzzle(&state, &user, &key).await?;
    let now = chrono::Utc::now().fixed_offset();
    let is_public = form.is_public.is_some();

    let mut active: puzzle::ActiveModel = model.clone().into();
    active.title = Set(blank_to_none(form.title));
    active.notes = Set(blank_to_none(form.notes));
    active.difficulty = Set(difficulty);
    active.region = Set(Region::parse(&form.region));
    active.themed = Set(form.themed.is_some());
    active.is_cryptic = Set(form.is_cryptic.is_some());
    active.is_public = Set(is_public);
    active.updated_at = Set(now);

    if is_public && model.published_at.is_none() {
        active.published_at = Set(Some(now));
    }
    if !is_public {
        active.featured_at = Set(None);
    }

    let saved = active.update(&state.db).await?;
    Ok(views::app::edit::save_response(&saved, &author, &user).into_response())
}

pub async fn toggle_feature(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    if !user.is_admin {
        return Err(AppError::NotFound);
    }

    let (model, _) = find_puzzle(&state, &user, &key).await?;
    let featured_at = (model.is_public && model.featured_at.is_none()).then(|| chrono::Utc::now().fixed_offset());

    let mut active: puzzle::ActiveModel = model.into();
    active.featured_at = Set(featured_at);

    let saved = active.update(&state.db).await?;
    Ok(views::app::edit::feature_block(&saved).into_response())
}

pub async fn reset_share(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    set_share(&state, &user, &key, Some(ids::new())).await
}

pub async fn remove_share(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    set_share(&state, &user, &key, None).await
}

async fn set_share(state: &AppState, user: &user::Model, key: &str, token: Option<String>) -> AppResult {
    let (model, author) = find_puzzle(state, user, key).await?;

    let mut active: puzzle::ActiveModel = model.into();
    active.share_token = Set(token);

    let saved = active.update(&state.db).await?;
    Ok(views::app::edit::share_block(&saved, &author).into_response())
}

pub async fn delete(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
    headers: HeaderMap,
) -> AppResult {
    let (model, _) = find_puzzle(&state, &user, &key).await?;
    model.delete(&state.db).await?;

    Ok(if htmx::is_htmx(&headers) {
        StatusCode::OK.into_response()
    } else {
        Redirect::to("/app").into_response()
    })
}
