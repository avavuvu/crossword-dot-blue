use axum::extract::{Multipart, State};
use boutique::{AuthenticatedUser, htmx};

use crate::{
    AppState,
    error::{AppError, AppResult},
    handlers::form::{UploadError, single_file},
    models::{puzzle, user},
};

pub async fn create(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult {
    let file = single_file(&mut multipart, "file").await.map_err(|error| match error {
        UploadError::Missing => AppError::field("file", "Choose a puzzle file to upload"),
        UploadError::Unreadable => AppError::field("file", "Something went wrong reading the upload"),
    })?;

    let extension = file
        .file_name
        .as_deref()
        .and_then(|name| name.rsplit_once('.'))
        .map(|(_, extension)| extension.to_string())
        .ok_or_else(|| AppError::field("file", "The file needs a .ipuz, .puz or .xd extension"))?;

    let parsed = crossword_tools::parse(&extension, &file.bytes)
        .map_err(|error| AppError::field("file", error.to_string()))?;

    let saved = puzzle::create(&state.db, &user.id, &parsed).await?;
    Ok(htmx::redirect(&saved.edit_path()))
}
