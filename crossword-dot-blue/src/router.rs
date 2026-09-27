use axum::{Router, extract::DefaultBodyLimit, routing::{get, post}};
use crate::{AppState, handlers::{account, app, auth, browse, crossword, embed, lander, options, profile, progress, text}};

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/",
            get(lander::show))
        .route("/about",
            get(text::about))
        .route("/privacy",
            get(text::privacy))
        .route("/login",
            get(auth::session::show)
            .post(auth::session::create))
        .route("/logout",
            post(auth::session::delete))
        .route("/signup",
            get(auth::signup::show)
            .post(auth::signup::create))
        .route("/crossword/{slug}",
            get(crossword::show))
        .route("/embed/{slug}",
            get(embed::show))
        .route("/progress/{key}",
            get(progress::show)
            .put(progress::save)
            .delete(progress::delete))
        .route("/options",
            get(options::show)
            .post(options::update))
        .route("/browse",
            get(browse::index))
        .route("/browse/{category}",
            get(browse::index))
        .route("/@{username}",
            get(profile::show))
        .route("/account",
            get(account::show)
            .post(account::update))
        .route("/account/avatar",
            post(account::upload_avatar)
            .layer(DefaultBodyLimit::max(account::AVATAR_BODY_LIMIT)))
        .route("/app",
            get(app::index::show))
        .route("/app/upload",
            post(app::upload::create))
        .route("/app/edit/{key}",
            get(app::edit::show)
            .post(app::edit::update))
        .route("/app/edit/{key}/share",
            post(app::edit::reset_share)
            .delete(app::edit::remove_share))
        .route("/app/edit/{key}/feature",
            post(app::edit::toggle_feature))
        .route("/app/edit/{key}/delete",
            post(app::edit::delete))
        .route("/app/edit/{key}/clues/{clue_id}",
            post(app::edit::clues::update))
        .route("/app/edit/{key}/clues/{clue_id}/split/{position}",
            post(app::edit::clues::toggle_split))
        .route("/app/edit/{key}/clues/{clue_id}/refs/{ref_id}/remove",
            post(app::edit::clues::remove_ref));

    router.with_state(state)
}
