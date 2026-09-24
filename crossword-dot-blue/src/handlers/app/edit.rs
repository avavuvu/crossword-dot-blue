pub mod clues;

use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use boutique::{AuthenticatedUser, htmx, validator::Validate};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{puzzle::{self, Region}, user},
    views,
};

#[derive(Deserialize, Validate)]
pub struct EditForm {
    #[validate(length(max = 200, message = "Title must be 200 characters or fewer"))]
    pub title: String,
    pub notes: String,
    pub difficulty: String,
    #[validate(length(max = 16, message = "Region must be 16 characters or fewer"))]
    pub region: String,
    #[serde(default)]
    pub themed: Option<String>,
    #[serde(default)]
    pub is_cryptic: Option<String>,
    #[serde(default)]
    pub is_public: Option<String>,
}

pub(super) async fn find_puzzle(state: &AppState, user: &user::Model, key: &str) -> AppResult<puzzle::Model> {
    if !puzzle::is_key(key) {
        return Err(AppError::NotFound);
    }

    let mut query = puzzle::Entity::find().filter(puzzle::Column::Id.starts_with(key));
    if !user.is_admin {
        query = query.filter(puzzle::Column::AuthorId.eq(&user.id));
    }

    query.one(&state.db).await?.ok_or(AppError::NotFound)
}

pub(super) async fn find_author(state: &AppState, user: &user::Model, model: &puzzle::Model) -> AppResult<user::Model> {
    if model.author_id == user.id {
        return Ok(user.clone());
    }

    model
        .find_related(user::Entity)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn edit_page(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    let model = find_puzzle(&state, &user, &key).await?;
    let author = find_author(&state, &user, &model).await?;
    let content = model.content()?;
    Ok(views::app::edit::edit(&model, &author, &user, &content).into_response())
}

pub async fn edit(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
    Form(form): Form<EditForm>,
) -> AppResult {
    if let Err(errors) = form.validate() {
        return Ok(htmx::fragments::from_errors(errors).into_response());
    }

    let difficulty = match form.difficulty.trim() {
        "" => None,
        value => match value.parse::<i16>() {
            Ok(d) if (1..=5).contains(&d) => Some(d),
            _ => return Ok(field_error("difficulty", "Difficulty must be a number from 1 to 5")),
        },
    };

    let existing = find_puzzle(&state, &user, &key).await?;
    let author = find_author(&state, &user, &existing).await?;
    let now = chrono::Utc::now().fixed_offset();
    let is_public = form.is_public.is_some();

    let mut active: puzzle::ActiveModel = existing.clone().into();
    active.title = Set(blank_to_none(form.title));
    active.notes = Set(blank_to_none(form.notes));
    active.difficulty = Set(difficulty);
    active.region = Set(Region::parse(&form.region));
    active.themed = Set(form.themed.is_some());
    active.is_cryptic = Set(form.is_cryptic.is_some());
    active.is_public = Set(is_public);
    active.updated_at = Set(now);

    if is_public && existing.published_at.is_none() {
        active.published_at = Set(Some(now));
    }
    if !is_public {
        active.featured_at = Set(None);
    }

    match active.update(&state.db).await {
        Ok(saved) => Ok(views::app::edit::save_response(&saved, &author, &user).into_response()),
        Err(e) => {
            eprintln!("[edit] {e}");
            Ok(htmx::fragments::error("Something went wrong saving the puzzle").into_response())
        }
    }
}

pub async fn toggle_feature(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    if !user.is_admin {
        return Err(AppError::NotFound);
    }

    let existing = find_puzzle(&state, &user, &key).await?;
    let featured_at = match (existing.is_public, existing.featured_at) {
        (false, _) => None,
        (true, Some(_)) => None,
        (true, None) => Some(chrono::Utc::now().fixed_offset()),
    };

    let mut active: puzzle::ActiveModel = existing.into();
    active.featured_at = Set(featured_at);

    let saved = active.update(&state.db).await?;
    Ok(views::app::edit::feature_block(&saved).into_response())
}

pub async fn reset_share(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    set_share(&state, &user, &key, Some(puzzle::new_share_token())).await
}

pub async fn remove_share(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    set_share(&state, &user, &key, None).await
}

async fn set_share(state: &AppState, user: &user::Model, key: &str, token: Option<String>) -> AppResult {
    let existing = find_puzzle(state, user, key).await?;
    let author = find_author(state, user, &existing).await?;

    let mut active: puzzle::ActiveModel = existing.into();
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
    let existing = find_puzzle(&state, &user, &key).await?;
    existing.delete(&state.db).await?;

    Ok(if htmx::is_htmx(&headers) {
        StatusCode::OK.into_response()
    } else {
        Redirect::to("/app").into_response()
    })
}

fn blank_to_none(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

fn field_error(field: &str, message: &str) -> Response {
    htmx::fragments::field_errors(&[(field, Some(message))]).into_response()
}
