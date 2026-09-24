use axum::{Router, extract::DefaultBodyLimit, routing::{get, post}};
use crate::{AppState, handlers};

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/",
            get(handlers::lander::lander))
        .route("/about",
            get(handlers::text::about))
        .route("/privacy",
            get(handlers::text::privacy))
        .route("/login",
            get(handlers::auth::login_page)
            .post(handlers::auth::login))
        .route("/signup",
            get(handlers::auth::signup_page)
            .post(handlers::auth::signup))
        .route("/logout",
            post(handlers::auth::logout))
        .route("/crossword/{slug}",
            get(handlers::crossword::show))
        .route("/browse",
            get(handlers::browse::index))
        .route("/browse/{category}",
            get(handlers::browse::category))
        .route("/@{username}",
            get(handlers::profile::show))
        .route("/settings",
            get(handlers::settings::page)
            .post(handlers::settings::update))
        .route("/settings/avatar",
            post(handlers::settings::avatar)
            .layer(DefaultBodyLimit::max(handlers::settings::AVATAR_MAX_BYTES + 64 * 1024)))
        .route("/app",
            get(handlers::app::index))
        .route("/app/upload",
            post(handlers::app::upload))
        .route("/app/edit/{key}",
            get(handlers::app::edit_page)
            .post(handlers::app::edit))
        .route("/app/edit/{key}/share",
            post(handlers::app::edit::reset_share)
            .delete(handlers::app::edit::remove_share))
        .route("/app/edit/{key}/feature",
            post(handlers::app::edit::toggle_feature))
        .route("/app/edit/{key}/delete",
            post(handlers::app::edit::delete))
        .route("/app/edit/{key}/clues/{clue_id}",
            post(handlers::app::edit::clues::update_clue))
        .route("/app/edit/{key}/clues/{clue_id}/split/{position}",
            post(handlers::app::edit::clues::toggle_split))
        .route("/app/edit/{key}/clues/{clue_id}/refs/{ref_id}/remove",
            post(handlers::app::edit::clues::remove_ref));

    router.with_state(state)
}
