use axum::extract::{Multipart, State};
use boutique::{AuthenticatedUser, htmx};
use crossword_tools::xd;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};

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
    let file = single_file(&mut multipart, "file").await.map_err(|e| match e {
        UploadError::Missing => AppError::field("file", "Choose a puzzle file to upload"),
        UploadError::Unreadable => AppError::field("file", "Something went wrong reading the upload"),
    })?;

    let extension = file
        .file_name
        .as_deref()
        .and_then(|name| name.rsplit_once('.'))
        .map(|(_, ext)| ext.to_string())
        .ok_or_else(|| AppError::field("file", "The file needs a .ipuz, .puz or .xd extension"))?;

    let parsed = crossword_tools::parse(&extension, &file.bytes)
        .map_err(|e| AppError::field("file", e.to_string()))?;

    let now = chrono::Utc::now().fixed_offset();
    let new_model = puzzle::ActiveModel {
        id: Set(unused_id(&state).await?),
        author_id: Set(user.id),
        created_at: Set(now),
        updated_at: Set(now),
        content: Set(serde_json::to_value(&parsed)?),
        xd: Set(xd::write::write_xd(&parsed)),
        title: Set(parsed.meta.title),
        notes: Set(parsed.meta.notes),
        difficulty: Set(None),
        region: Set(None),
        themed: Set(false),
        is_cryptic: Set(false),
        is_public: Set(false),
        published_at: Set(None),
        featured_at: Set(None),
        share_token: Set(Some(puzzle::new_share_token())),
    };

    let saved = new_model.insert(&state.db).await?;
    Ok(htmx::redirect(&saved.edit_path()))
}

async fn unused_id(state: &AppState) -> Result<String, DbErr> {
    loop {
        let id = puzzle::new_id();
        let taken = puzzle::Entity::find()
            .filter(puzzle::Column::Id.starts_with(&id[..puzzle::KEY_LEN]))
            .one(&state.db)
            .await?
            .is_some();

        if !taken {
            return Ok(id);
        }
    }
}
