pub mod clues;

use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use boutique::{AuthenticatedUser, htmx, validator::Validate};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    AppState,
    error::{AppError, AppResult},
    handlers::form::blank_to_none,
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

pub async fn show(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> AppResult {
    let model = find_puzzle(&state, &user, &key).await?;
    let author = find_author(&state, &user, &model).await?;
    let puzzle = model.puzzle()?;
    Ok(views::app::edit::page(&model, &author, &user, &puzzle).into_response())
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

    let model = find_puzzle(&state, &user, &key).await?;
    let author = find_author(&state, &user, &model).await?;
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

    let model = find_puzzle(&state, &user, &key).await?;
    let featured_at = match (model.is_public, model.featured_at) {
        (false, _) => None,
        (true, Some(_)) => None,
        (true, None) => Some(chrono::Utc::now().fixed_offset()),
    };

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
    let model = find_puzzle(state, user, key).await?;
    let author = find_author(state, user, &model).await?;

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
    let model = find_puzzle(&state, &user, &key).await?;
    model.delete(&state.db).await?;

    Ok(if htmx::is_htmx(&headers) {
        StatusCode::OK.into_response()
    } else {
        Redirect::to("/app").into_response()
    })
}
