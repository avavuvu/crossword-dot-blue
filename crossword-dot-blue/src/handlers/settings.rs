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
    views,
};

pub const AVATAR_MAX_BYTES: usize = 5 * 1024 * 1024;
const AVATAR_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/webp"];

#[derive(Deserialize, Validate)]
pub struct SettingsForm {
    #[validate(length(max = 60, message = "Display name must be 60 characters or fewer"))]
    pub display_name: String,
    #[validate(length(max = 1000, message = "Bio must be 1000 characters or fewer"))]
    pub bio: String,
}

pub async fn show(AuthenticatedUser(user): AuthenticatedUser<user::Model>) -> AppResult {
    Ok(views::settings::page(&user).into_response())
}

pub async fn update(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Form(form): Form<SettingsForm>,
) -> AppResult {
    form.validate()?;

    let mut active: user::ActiveModel = user.into();
    active.display_name = Set(blank_to_none(form.display_name));
    active.bio = Set(blank_to_none(form.bio));
    active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

    let saved = active.update(&state.db).await?;
    Ok(views::settings::save_status(&saved).into_response())
}

pub async fn upload_avatar(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult {
    if !cloudinary::is_configured() {
        return Err(AppError::field("avatar", "Image uploads are not available right now"));
    }

    let file = single_file(&mut multipart, "avatar").await.map_err(|e| match e {
        UploadError::Missing => AppError::field("avatar", "Choose an image to upload"),
        UploadError::Unreadable => AppError::field("avatar", "The image is too large or could not be read"),
    })?;

    let content_type = file.content_type.unwrap_or_default();
    if !AVATAR_TYPES.contains(&content_type.as_str()) {
        return Err(AppError::field("avatar", "The image needs to be a JPEG, PNG or WebP"));
    }

    let bytes = file.bytes;
    if bytes.len() > AVATAR_MAX_BYTES {
        return Err(AppError::field("avatar", "The image needs to be 5 MB or smaller"));
    }

    if !looks_like_image(&bytes) {
        return Err(AppError::field("avatar", "The file does not look like an image"));
    }

    let public_id = cloudinary::upload(bytes.to_vec(), &user.avatar_public_id(), &content_type)
        .await
        .map_err(|e| {
            eprintln!("[avatar] {e}");
            AppError::field("avatar", "Something went wrong uploading the image")
        })?;

    let mut active: user::ActiveModel = user.into();
    active.avatar_public_id = Set(Some(public_id));
    active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

    let saved = active.update(&state.db).await?;
    Ok(views::settings::avatar_block(&saved).into_response())
}

fn looks_like_image(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8, 0xFF])
        || bytes.starts_with(&[0x89, b'P', b'N', b'G'])
        || (bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP"))
}
