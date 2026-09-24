use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{UserContext, htmx, session::{self, LoginError}};
use boutique::validator::Validate;
use serde::Deserialize;

use crate::{AppState, views};

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

pub async fn login_page(
    Extension(ctx): Extension<UserContext>,
) -> Response {
    if ctx.is_authenticated() {
        return Redirect::to("/app").into_response();
    }

    views::auth::login().into_response()
}

pub async fn login(State(auth): State<AppState>, Form(form): Form<LoginForm>) -> Response {
    match session::login(&auth, &form.email, &form.password).await {
        Ok((_user, session)) => (session, htmx::redirect("/")).into_response(),
        Err(LoginError::InvalidCredentials) => htmx::fragments::error("Incorrect email or password").into_response(),
        Err(LoginError::Database(e)) => htmx::fragments::error(&e.to_string()).into_response(),
        Err(_) => htmx::fragments::error("Something went wrong").into_response(),
    }
}
