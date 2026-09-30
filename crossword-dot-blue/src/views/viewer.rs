use axum::{extract::FromRequestParts, http::request::Parts};
use boutique::{AppError, AuthenticatedUser};

use crate::{AppState, models::user, theme::Theme};

#[derive(Clone, Debug, Default)]
pub struct Viewer {
    pub user_id: Option<String>,
    pub theme: Theme,
}

impl Viewer {
    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }
}

impl FromRequestParts<AppState> for Viewer {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let user = Option::<AuthenticatedUser<user::Model>>::from_request_parts(parts, state).await?;
        Ok(Viewer { user_id: user.map(|AuthenticatedUser(user)| user.id), theme: Theme::from_headers(&parts.headers) })
    }
}
