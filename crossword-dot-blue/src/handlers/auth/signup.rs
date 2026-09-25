use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect}};
use boutique::{UserContext, htmx, session};
use boutique::validator::{Validate, ValidationError};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{AppState, error::{AppError, AppResult}, models::user, views};

fn alphanumeric(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        let mut e = ValidationError::new("alphanumeric");
        e.message = Some("Username can only contain letters and numbers".into());
        Err(e)
    }
}

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(
        custom(function = "alphanumeric", message = "Username can only contain letters and numbers"),
        length(min = 3, max = 20, message = "Username must be between 3 and 20 characters")
    )]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

pub async fn show(Extension(ctx): Extension<UserContext>) -> AppResult {
    if ctx.is_authenticated() {
        return Ok(Redirect::to("/app").into_response());
    }

    Ok(views::auth::signup::page().into_response())
}

pub async fn create(State(state): State<AppState>, Form(form): Form<SignupForm>) -> AppResult {
    form.validate()?;

    let email_taken = user::Entity::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await?
        .is_some();

    let username_taken = user::Entity::find()
        .filter(user::Column::Username.eq(&form.username))
        .one(&state.db)
        .await?
        .is_some();

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

    let is_admin = crate::is_admin_email(&form.email);
    let new_user = user::new(&form.email, &form.username, &form.password, is_admin)
        .map_err(|e| AppError::internal("password hash", e))?;
    let saved = new_user.insert(&state.db).await?;

    let session = session::issue(&state, &saved)
        .await
        .map_err(|e| AppError::internal("session", format!("{e:?}")))?;

    Ok((session, htmx::redirect("/app")).into_response())
}
