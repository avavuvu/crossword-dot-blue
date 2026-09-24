use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{UserContext, htmx, session};
use boutique::validator::Validate;
use serde::Deserialize;

use crate::{AppState, models::user::{self, Entity as User}, views};

fn alphanumeric(value: &str) -> Result<(), boutique::validator::ValidationError> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        let mut e = boutique::validator::ValidationError::new("alphanumeric");
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

fn something_went_wrong() -> Response {
    htmx::fragments::error("Something went wrong, please try again").into_response()
}

pub async fn signup_page(
    Extension(ctx): Extension<UserContext>,
) -> Response {
    if ctx.is_authenticated() {
        return Redirect::to("/app").into_response();
    }

    views::auth::signup().into_response()
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::fragments::from_errors(errors).into_response();
    }

    let email_taken = User::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await
        .unwrap()
        .is_some();

    let username_taken = User::find()
        .filter(user::Column::Username.eq(&form.username))
        .one(&state.db)
        .await
        .unwrap()
        .is_some();

    if email_taken || username_taken {
        return htmx::fragments::field_errors(&[
            ("email", email_taken.then_some("An account with this email already exists")),
            ("username", username_taken.then_some("This username is already taken")),
        ]).into_response();
    }

    let is_admin = crate::is_admin_email(&form.email);
    let new_user = match user::new(&form.email, &form.username, &form.password, is_admin) {
        Ok(user) => user,
        Err(e) => {
            eprintln!("[signup] {e}");
            return something_went_wrong();
        }
    };

    let user = match new_user.insert(&state.db).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("[signup] {e}");
            return something_went_wrong();
        }
    };

    match session::issue(&state, &user).await {
        Ok(session) => (session, htmx::redirect("/app")).into_response(),
        Err(e) => {
            eprintln!("[signup] {e:?}");
            something_went_wrong()
        }
    }
}
