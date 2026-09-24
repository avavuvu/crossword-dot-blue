use axum::{extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::session::{self, CookieJar};

use crate::AppState;

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Response {
    let session = session::revoke(&state, jar).await;
    (session, Redirect::to("/")).into_response()
}
