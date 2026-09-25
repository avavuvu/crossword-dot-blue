use axum::{
    Form,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use boutique::AuthenticatedUser;
use crossword_tools::{puzzle::Puzzle, xd};
use maud::html;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Deserialize;

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{puzzle::ActiveModel, user},
    views::app::edit::{clues::{clue_row, ref_label}, grid_preview},
};

use super::find_puzzle;

#[derive(Deserialize)]
pub struct ClueForm {
    pub body: String,
    #[serde(default)]
    pub add_ref: String,
}

enum Action {
    Save,
    ToggleSplit(usize),
    RemoveRef(String),
}

pub async fn update(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path((key, clue_id)): Path<(String, String)>,
    Form(form): Form<ClueForm>,
) -> AppResult {
    save_clue(&state, &user, &key, &clue_id, form, Action::Save).await
}

pub async fn toggle_split(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path((key, clue_id, position)): Path<(String, String, usize)>,
    Form(form): Form<ClueForm>,
) -> AppResult {
    save_clue(&state, &user, &key, &clue_id, form, Action::ToggleSplit(position)).await
}

pub async fn remove_ref(
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    State(state): State<AppState>,
    Path((key, clue_id, ref_id)): Path<(String, String, String)>,
    Form(form): Form<ClueForm>,
) -> AppResult {
    save_clue(&state, &user, &key, &clue_id, form, Action::RemoveRef(ref_id)).await
}

async fn save_clue(
    state: &AppState,
    user: &user::Model,
    key: &str,
    clue_id: &str,
    form: ClueForm,
    action: Action,
) -> AppResult {
    let model = find_puzzle(state, user, key).await?;
    let mut puzzle = model.puzzle()?;

    if !puzzle.clues.contains_key(clue_id) {
        return Err(AppError::NotFound);
    }

    let error_row = |puzzle: &Puzzle, message: &str| -> Response {
        clue_row(key, puzzle, &puzzle.clues[clue_id], Some(message), false).into_response()
    };

    let body = form.body.split_whitespace().collect::<Vec<_>>().join(" ");
    let body = body.as_str();
    if body.is_empty() {
        return Ok(error_row(&puzzle, "Clue text is required"));
    }

    let mut linked: Option<String> = None;
    let split_changed = matches!(action, Action::ToggleSplit(_));

    match action {
        Action::Save => {}
        Action::ToggleSplit(position) => {
            let clue = puzzle.clues.get_mut(clue_id).unwrap();
            if position == 0 || position >= clue.indexes.len() {
                return Err(AppError::BadRequest);
            }
            match clue.splits.iter().position(|&p| p == position) {
                Some(existing) => {
                    clue.splits.remove(existing);
                }
                None => {
                    clue.splits.push(position);
                    clue.splits.sort_unstable();
                }
            }
        }
        Action::RemoveRef(ref_id) => {
            unlink(&mut puzzle, clue_id, &ref_id);
            linked = Some(ref_id);
        }
    }

    if !form.add_ref.trim().is_empty() {
        let Some(ref_id) = parse_ref(&form.add_ref) else {
            return Ok(error_row(&puzzle, "Enter a clue like 12 Down or D12"));
        };
        if ref_id == clue_id {
            return Ok(error_row(&puzzle, "A clue cannot link to itself"));
        }
        if !puzzle.clues.contains_key(&ref_id) {
            return Ok(error_row(&puzzle, &format!("There is no clue {}", ref_label(&ref_id))));
        }
        link(&mut puzzle, clue_id, &ref_id);
        linked = Some(ref_id);
    }

    puzzle.clues.get_mut(clue_id).unwrap().body = body.to_string();

    let mut active: ActiveModel = model.into();
    active.content = Set(serde_json::to_value(&puzzle)?);
    active.xd = Set(xd::write::write_xd(&puzzle));
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    if let Err(e) = active.update(&state.db).await {
        eprintln!("[clues] {e}");
        return Ok(error_row(&puzzle, "Something went wrong saving the clue"));
    }

    let other = linked.and_then(|id| puzzle.clues.get(&id));

    Ok(html! {
        (clue_row(key, &puzzle, &puzzle.clues[clue_id], None, false))
        @if let Some(other) = other {
            (clue_row(key, &puzzle, other, None, true))
        }
        @if split_changed {
            (grid_preview(&puzzle, true))
        }
    }
    .into_response())
}

fn link(puzzle: &mut Puzzle, from: &str, to: &str) {
    for (a, b) in [(from, to), (to, from)] {
        if let Some(clue) = puzzle.clues.get_mut(a) {
            if !clue.refs.iter().any(|r| r == b) {
                clue.refs.push(b.to_string());
            }
        }
    }
}

fn unlink(puzzle: &mut Puzzle, from: &str, to: &str) {
    for (a, b) in [(from, to), (to, from)] {
        if let Some(clue) = puzzle.clues.get_mut(a) {
            clue.refs.retain(|r| r != b);
        }
    }
}

fn parse_ref(text: &str) -> Option<String> {
    let text = text.trim().to_ascii_uppercase();

    if let Some(number) = text.strip_prefix('A').or_else(|| text.strip_prefix('D')) {
        if !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()) {
            return Some(text);
        }
    }

    let digits: String = text.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }

    let direction = match text[digits.len()..].trim_matches([' ', '-']) {
        "A" | "ACROSS" => 'A',
        "D" | "DOWN" => 'D',
        _ => return None,
    };

    Some(format!("{direction}{digits}"))
}
