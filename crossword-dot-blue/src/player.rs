use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponseParts, ResponseParts},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use boutique::{UserContext, session::CookieJar};

use crate::{AppState, ids};

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
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let user_id = parts.extensions.get::<UserContext>().and_then(|context| context.user_id.clone());

        let (id, fresh) = match jar.get(COOKIE).map(|cookie| cookie.value()).filter(|value| ids::is_hex(value, ids::LEN)) {
            Some(existing) => (existing.to_string(), false),
            None => (ids::new(), true),
        };

        Ok(Player { id, user_id, fresh, secure: state.config.secure_cookies })
    }
}
