use axum::{extract::FromRequestParts, http::request::Parts};
use boutique::UserContext;

use crate::{AppState, theme::Theme};

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
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self, Self::Rejection> {
        let user_id = parts.extensions.get::<UserContext>().and_then(|context| context.user_id.clone());
        Ok(Viewer { user_id, theme: Theme::from_headers(&parts.headers) })
    }
}
