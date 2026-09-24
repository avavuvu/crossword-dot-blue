use axum::{
    extract::{Multipart, State},
    response::{IntoResponse, Response},
};
use boutique::{AuthenticatedUser, htmx};
use crossword_tools::xd;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::{AppState, error::AppResult, models::{puzzle, user}};

pub async fn upload(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult {
    let file = loop {
        match multipart.next_field().await {
            Ok(Some(field)) if field.name() == Some("file") => break field,
            Ok(Some(_)) => continue,
            Ok(None) => return Ok(file_error("Choose a puzzle file to upload")),
            Err(e) => {
                eprintln!("[upload] {e}");
                return Ok(file_error("Something went wrong reading the upload"));
            }
        }
    };

    let Some(extension) = file
        .file_name()
        .and_then(|name| name.rsplit_once('.'))
        .map(|(_, ext)| ext.to_string())
    else {
        return Ok(file_error("The file needs a .ipuz, .puz or .xd extension"));
    };

    let bytes = match file.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[upload] {e}");
            return Ok(file_error("Something went wrong reading the upload"));
        }
    };

    let puzzle = match crossword_tools::parse(&extension, &bytes) {
        Ok(puzzle) => puzzle,
        Err(e) => return Ok(file_error(&e.to_string())),
    };

    let now = chrono::Utc::now().fixed_offset();
    let new_puzzle = puzzle::ActiveModel {
        id: Set(unused_id(&state).await?),
        author_id: Set(user.id),
        created_at: Set(now),
        updated_at: Set(now),
        content: Set(serde_json::to_value(&puzzle)?),
        xd: Set(xd::write::write_xd(&puzzle)),
        title: Set(puzzle.meta.title),
        notes: Set(puzzle.meta.notes),
        difficulty: Set(None),
        region: Set(None),
        themed: Set(false),
        is_cryptic: Set(false),
        is_public: Set(false),
        published_at: Set(None),
        featured_at: Set(None),
        share_token: Set(Some(puzzle::new_share_token())),
    };

    let saved = new_puzzle.insert(&state.db).await?;
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

fn file_error(message: &str) -> Response {
    htmx::fragments::field_errors(&[("file", Some(message))]).into_response()
}
