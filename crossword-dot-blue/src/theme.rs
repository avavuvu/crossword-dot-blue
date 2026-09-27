use axum::http::HeaderMap;
use axum_extra::extract::cookie::{Cookie, SameSite};
use boutique::session::CookieJar;

include!(concat!(env!("OUT_DIR"), "/themes.rs"));

pub const COOKIE: &str = "theme";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Theme {
    name: &'static str,
}

impl Theme {
    pub fn parse(value: &str) -> Option<Theme> {
        THEMES.iter().find(|(name, _)| *name == value).map(|(name, _)| Theme { name })
    }

    pub fn from_headers(headers: &HeaderMap) -> Theme {
        CookieJar::from_headers(headers)
            .get(COOKIE)
            .and_then(|cookie| Theme::parse(cookie.value()))
            .unwrap_or_default()
    }

    pub fn name(self) -> &'static str {
        self.name
    }

    pub fn attribute(self) -> Option<&'static str> {
        (!self.name.is_empty()).then_some(self.name)
    }

    pub fn cookie(self, secure: bool) -> CookieJar {
        let cookie = Cookie::build((COOKIE, self.name))
            .path("/")
            .secure(secure)
            .same_site(SameSite::Lax)
            .max_age(time::Duration::days(365))
            .build();
        CookieJar::new().add(cookie)
    }
}
