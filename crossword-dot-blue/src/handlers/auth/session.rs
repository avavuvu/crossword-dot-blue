use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect}};
use boutique::{UserContext, htmx, session::{self, CookieJar, LoginError}};
use boutique::validator::Validate;
use serde::Deserialize;

use crate::{AppState, error::{AppError, AppResult}, views};

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

pub async fn show(Extension(ctx): Extension<UserContext>) -> AppResult {
    if ctx.is_authenticated() {
        return Ok(Redirect::to("/app").into_response());
    }

    Ok(views::auth::login::page().into_response())
}

pub async fn create(State(state): State<AppState>, Form(form): Form<LoginForm>) -> AppResult {
    match session::login(&state, &form.email, &form.password).await {
        Ok((_user, session)) => Ok((session, htmx::redirect("/")).into_response()),
        Err(LoginError::InvalidCredentials) => Err(AppError::message("Incorrect email or password")),
        Err(LoginError::Database(e)) => Err(AppError::internal("login", e)),
        Err(e) => Err(AppError::internal("login", format!("{e:?}"))),
    }
}

pub async fn delete(State(state): State<AppState>, jar: CookieJar) -> AppResult {
    let session = session::revoke(&state, jar).await;
    Ok((session, Redirect::to("/")).into_response())
}
