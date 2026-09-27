use axum::{
    Form,
    extract::{Multipart, State},
    response::IntoResponse,
};
use boutique::{AuthenticatedUser, validator::Validate};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Deserialize;

use crate::{
    AppState, cloudinary,
    error::{AppError, AppResult},
    handlers::form::{UploadError, blank_to_none, single_file},
    models::user,
    views::{self, viewer::Viewer},
};

const AVATAR_MAX_BYTES: usize = 5 * 1024 * 1024;
const MULTIPART_OVERHEAD: usize = 64 * 1024;
pub const AVATAR_BODY_LIMIT: usize = AVATAR_MAX_BYTES + MULTIPART_OVERHEAD;

#[derive(Deserialize, Validate)]
pub struct AccountSettingsForm {
    #[validate(length(max = 60, message = "Display name must be 60 characters or fewer"))]
    pub display_name: String,
    #[validate(length(max = 1000, message = "Bio must be 1000 characters or fewer"))]
    pub bio: String,
}

pub async fn show(AuthenticatedUser(user): AuthenticatedUser<user::Model>, viewer: Viewer) -> AppResult {
    Ok(views::account::page(&user, &viewer).into_response())
}

pub async fn update(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Form(form): Form<AccountSettingsForm>,
) -> AppResult {
    form.validate()?;

    let mut active: user::ActiveModel = user.into();
    active.display_name = Set(blank_to_none(form.display_name));
    active.bio = Set(blank_to_none(form.bio));
    active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

    let saved = active.update(&state.db).await?;
    Ok(views::account::save_status(&saved).into_response())
}

pub async fn upload_avatar(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult {
    if !cloudinary::is_configured() {
        return Err(AppError::field("avatar", "Image uploads are not available right now"));
    }

    let file = single_file(&mut multipart, "avatar").await.map_err(|error| match error {
        UploadError::Missing => AppError::field("avatar", "Choose an image to upload"),
        UploadError::Unreadable => AppError::field("avatar", "The image is too large or could not be read"),
    })?;

    let bytes = file.bytes;
    if bytes.len() > AVATAR_MAX_BYTES {
        return Err(AppError::field("avatar", "The image needs to be 5 MB or smaller"));
    }

    let content_type =
        image_type(&bytes).ok_or_else(|| AppError::field("avatar", "The image needs to be a JPEG, PNG or WebP"))?;

    let public_id = cloudinary::upload(bytes.to_vec(), &user.avatar_upload_id(), content_type)
        .await
        .map_err(|error| {
            eprintln!("[avatar] {error}");
            AppError::field("avatar", "Something went wrong uploading the image")
        })?;

    let mut active: user::ActiveModel = user.into();
    active.avatar_public_id = Set(Some(public_id));
    active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

    let saved = active.update(&state.db).await?;
    Ok(views::account::avatar_block(&saved).into_response())
}

fn image_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("image/jpeg")
    } else if bytes.starts_with(&[0x89, b'P', b'N', b'G']) {
        Some("image/png")
    } else if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
        Some("image/webp")
    } else {
        None
    }
}
