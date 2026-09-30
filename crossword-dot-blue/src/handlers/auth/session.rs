use axum::{Form, extract::State, response::{IntoResponse, Redirect}};
use boutique::{AppError, AppResult, htmx, session::{self, CookieJar, LoginError}};
use boutique::validator::Validate;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, TransactionTrait};
use serde::Deserialize;

use crate::{AppState, config, models::{progress, user}, player::{self, Player, SetPlayer}, views::{self, viewer::Viewer}};

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

pub async fn show(viewer: Viewer) -> AppResult {
    if viewer.is_authenticated() {
        return Ok(Redirect::to("/app").into_response());
    }

    Ok(views::auth::login::page(&viewer).into_response())
}

pub async fn create(State(state): State<AppState>, player: Player, Form(form): Form<LoginForm>) -> AppResult {
    let email = config::normalize_email(&form.email);

    let (user, session) = match session::login(&state.auth, &email, &form.password).await {
        Ok(ok) => ok,
        Err(LoginError::InvalidCredentials) => return Err(AppError::message("Incorrect email or password")),
        Err(error) => return Err(AppError::internal("login", error)),
    };

    let transaction = state.db.begin().await?;
    progress::claim(&transaction, &player.id, &user.id).await?;
    promote_admin(&transaction, user).await?;
    transaction.commit().await?;

    Ok((session, SetPlayer(Some(player.rotate())), htmx::redirect("/app")).into_response())
}

pub async fn delete(State(state): State<AppState>, jar: CookieJar) -> AppResult {
    let session = session::revoke(&state.auth, jar).await;
    Ok((session, SetPlayer(Some(player::remove())), Redirect::to("/")).into_response())
}

async fn promote_admin(db: &impl sea_orm::ConnectionTrait, user: user::Model) -> Result<(), sea_orm::DbErr> {
    if user.is_admin || !config::is_admin_email(&user.email) {
        return Ok(());
    }

    let mut active: user::ActiveModel = user.into();
    active.is_admin = Set(true);
    active.update(db).await?;
    Ok(())
}
