use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponseParts, ResponseParts},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use boutique::{AppError, AuthenticatedUser, ids, session::CookieJar};

use crate::{AppState, models::user};

pub const COOKIE: &str = "cw_player";

pub struct Player {
    pub id: String,
    pub user_id: Option<String>,
    fresh: bool,
    secure: bool,
}

impl Player {
    pub fn cookie(&self) -> Option<CookieJar> {
        self.fresh.then(|| CookieJar::new().add(build(self.id.clone(), self.secure)))
    }

    pub fn rotate(&self) -> CookieJar {
        CookieJar::new().add(build(ids::new(), self.secure))
    }
}

pub fn remove() -> CookieJar {
    CookieJar::new().add(Cookie::build((COOKIE, "")).path("/").max_age(time::Duration::ZERO).build())
}

fn build(id: String, secure: bool) -> Cookie<'static> {
    Cookie::build((COOKIE, id))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(365))
        .build()
}

pub struct SetPlayer(pub Option<CookieJar>);

impl IntoResponseParts for SetPlayer {
    type Error = <CookieJar as IntoResponseParts>::Error;

    fn into_response_parts(self, parts: ResponseParts) -> Result<ResponseParts, Self::Error> {
        match self.0 {
            Some(jar) => jar.into_response_parts(parts),
            None => Ok(parts),
        }
    }
}

impl FromRequestParts<AppState> for Player {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let user_id = Option::<AuthenticatedUser<user::Model>>::from_request_parts(parts, state)
            .await?
            .map(|AuthenticatedUser(user)| user.id);

        let (id, fresh) = match jar.get(COOKIE).map(|cookie| cookie.value()).filter(|value| ids::is_hex(value, ids::LEN)) {
            Some(existing) => (existing.to_string(), false),
            None => (ids::new(), true),
        };

        Ok(Player { id, user_id, fresh, secure: state.auth.config.secure_cookies })
    }
}
