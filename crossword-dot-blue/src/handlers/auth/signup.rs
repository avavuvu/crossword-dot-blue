use axum::{Form, extract::State, response::{IntoResponse, Redirect}};
use boutique::{AppError, AppResult, htmx, session};
use boutique::validator::{Validate, ValidationError};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, SqlErr, TransactionTrait};
use serde::Deserialize;

use crate::{AppState, config, models::{progress, user}, player::{Player, SetPlayer}, views::{self, viewer::Viewer}};

fn ascii_alphanumeric(value: &str) -> Result<(), ValidationError> {
    if value.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        Err(ValidationError::new("ascii_alphanumeric"))
    }
}

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(
        custom(function = "ascii_alphanumeric", message = "Username can only contain letters and numbers"),
        length(min = 3, max = 20, message = "Username must be between 3 and 20 characters")
    )]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

pub async fn show(viewer: Viewer) -> AppResult {
    if viewer.is_authenticated() {
        return Ok(Redirect::to("/app").into_response());
    }

    Ok(views::auth::signup::page(&viewer).into_response())
}

pub async fn create(State(state): State<AppState>, player: Player, Form(form): Form<SignupForm>) -> AppResult {
    form.validate()?;
    let email = config::normalize_email(&form.email);
    let username = form.username.trim();

    let email_taken = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(&state.db)
        .await?
        .is_some();
    let username_taken = user::find_by_username(&state.db, username).await?.is_some();

    if email_taken || username_taken {
        let mut fields = Vec::new();
        if email_taken {
            fields.push(("email", "An account with this email already exists".to_string()));
        }
        if username_taken {
            fields.push(("username", "This username is already taken".to_string()));
        }
        return Err(AppError::Fields(fields));
    }

    let new_user = user::new(&email, username, &form.password, config::is_admin_email(&email))
        .map_err(|error| AppError::internal("password hash", error))?;

    let transaction = state.db.begin().await?;
    let saved = new_user.insert(&transaction).await.map_err(taken)?;
    progress::claim(&transaction, &player.id, &saved.id).await?;
    transaction.commit().await?;

    let session = session::issue(&state, &saved)
        .await
        .map_err(|error| AppError::internal("session", error))?;

    Ok((session, SetPlayer(Some(player.rotate())), htmx::redirect("/app")).into_response())
}

fn taken(error: DbErr) -> AppError {
    match error.sql_err() {
        Some(SqlErr::UniqueConstraintViolation(detail)) if detail.contains("email") => {
            AppError::field("email", "An account with this email already exists")
        }
        Some(SqlErr::UniqueConstraintViolation(_)) => AppError::field("username", "This username is already taken"),
        _ => error.into(),
    }
}
