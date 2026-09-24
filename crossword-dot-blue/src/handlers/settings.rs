use axum::{
    Form,
    extract::{Multipart, State},
    response::{IntoResponse, Response},
};
use boutique::{AuthenticatedUser, htmx, validator::Validate};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Deserialize;

use crate::{AppState, cloudinary, error::AppResult, models::user, views};

pub const AVATAR_MAX_BYTES: usize = 5 * 1024 * 1024;
const AVATAR_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/webp"];

#[derive(Deserialize, Validate)]
pub struct SettingsForm {
    #[validate(length(max = 60, message = "Display name must be 60 characters or fewer"))]
    pub display_name: String,
    #[validate(length(max = 1000, message = "Bio must be 1000 characters or fewer"))]
    pub bio: String,
}

pub async fn page(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> AppResult {
    Ok(views::settings::show(&user).into_response())
}

pub async fn update(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Form(form): Form<SettingsForm>,
) -> AppResult {
    if let Err(errors) = form.validate() {
        return Ok(htmx::fragments::from_errors(errors).into_response());
    }

    let mut active: user::ActiveModel = user.into();
    active.display_name = Set(blank_to_none(form.display_name));
    active.bio = Set(blank_to_none(form.bio));
    active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

    match active.update(&state.db).await {
        Ok(saved) => Ok(views::settings::save_status(&saved).into_response()),
        Err(e) => {
            eprintln!("[settings] {e}");
            Ok(htmx::fragments::error("Something went wrong saving your profile").into_response())
        }
    }
}

pub async fn avatar(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult {
    if !cloudinary::is_configured() {
        return Ok(file_error("Image uploads are not available right now"));
    }

    let file = loop {
        match multipart.next_field().await {
            Ok(Some(field)) if field.name() == Some("avatar") => break field,
            Ok(Some(_)) => continue,
            Ok(None) => return Ok(file_error("Choose an image to upload")),
            Err(e) => {
                eprintln!("[avatar] {e}");
                return Ok(file_error("Something went wrong reading the upload"));
            }
        }
    };

    let content_type = file.content_type().unwrap_or("").to_string();
    if !AVATAR_TYPES.contains(&content_type.as_str()) {
        return Ok(file_error("The image needs to be a JPEG, PNG or WebP"));
    }

    let bytes = match file.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[avatar] {e}");
            return Ok(file_error("The image is too large or could not be read"));
        }
    };

    if bytes.len() > AVATAR_MAX_BYTES {
        return Ok(file_error("The image needs to be 5 MB or smaller"));
    }

    if !looks_like_image(&bytes) {
        return Ok(file_error("The file does not look like an image"));
    }

    let public_id = match cloudinary::upload(bytes.to_vec(), &user.avatar_public_id(), &content_type).await {
        Ok(public_id) => public_id,
        Err(e) => {
            eprintln!("[avatar] {e}");
            return Ok(file_error("Something went wrong uploading the image"));
        }
    };

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

fn blank_to_none(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

fn file_error(message: &str) -> Response {
    htmx::fragments::field_errors(&[("avatar", Some(message))]).into_response()
}
