use std::collections::BTreeMap;

use crate::grid::Slot;
use crate::puzzle::{Cell, Clue, Direction};

pub fn generate_clue_id(direction: Direction, number: u16) -> String {
    let prefix = match direction {
        Direction::Across => 'A',
        Direction::Down => 'D',
    };
    format!("{prefix}{number}")
}

pub fn index_clues(clues: Vec<Clue>) -> (BTreeMap<String, Clue>, Vec<String>) {
    let order = clues.iter().map(|clue| clue.id.clone()).collect();
    let map = clues.into_iter().map(|clue| (clue.id.clone(), clue)).collect();
    (map, order)
}

pub fn trimmed_or_none(text: &str) -> Option<String> {
    let trimmed = text.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

#[derive(Debug, thiserror::Error)]
pub enum ShapeError {
    #[error("has {found} rows, expected {expected}")]
    RowCount { expected: usize, found: usize },
    #[error("row {row} has {found} cells, expected {expected}")]
    RowWidth { row: usize, expected: usize, found: usize },
}

pub fn check_shape<T>(
    rows: &[T],
    width: usize,
    height: usize,
    len: impl Fn(&T) -> usize,
) -> Result<(), ShapeError> {
    if rows.len() != height {
        return Err(ShapeError::RowCount { expected: height, found: rows.len() });
    }
    for (row, cells) in rows.iter().enumerate() {
        let found = len(cells);
        if found != width {
            return Err(ShapeError::RowWidth { row, expected: width, found });
        }
    }
    Ok(())
}

pub fn register_rebus(rebuses: &mut BTreeMap<String, String>, word: &str) {
    if word.chars().count() > 1 && !rebuses.values().any(|v| v == word) {
        rebuses.insert((rebuses.len() + 1).to_string(), word.to_string());
    }
}

/// sets the clue number on the first cell and the clue id on every cell of the slot.
/// returns the answer, which is the joined solution of the cells.
pub fn assign_slot(cells: &mut [Cell], slot: &Slot, id: &str) -> String {
    let mut answer = String::new();

    for (position, &index) in slot.indexes.iter().enumerate() {
        if let Cell::Letter { solution, number, clues, .. } = &mut cells[index] {
            answer.push_str(solution);
            if position == 0 {
                *number = Some(slot.number);
            }
            match slot.direction {
                Direction::Across => clues.across = Some(id.to_string()),
                Direction::Down => clues.down = Some(id.to_string()),
            }
        }
    }

    answer
}

pub fn slot_solutions<'a>(cells: &'a [Cell], slot: &Slot) -> Vec<&'a str> {
    slot.indexes
        .iter()
        .filter_map(|&i| match &cells[i] {
            Cell::Letter { solution, .. } => Some(solution.as_str()),
            Cell::Block { .. } => None,
        })
        .collect()
}

pub fn find_splits_in_clue(clue: &str, solutions: &[&str]) -> (String, Option<Vec<usize>>) {
    let regex = regex::Regex::new(r"\((?P<word_lengths>\d+(?:,\d+)+)\)$").unwrap();
    let clue = clue.trim();

    let Some(capture) = regex.captures(clue) else {
        return (clue.to_string(), None);
    };

    let mut boundaries = Vec::new();
    let mut letters = 0;
    for part in capture["word_lengths"].split(',') {
        let Ok(length) = part.parse::<usize>() else {
            return (clue.to_string(), None);
        };
        letters += length;
        boundaries.push(letters);
    }
    boundaries.pop();

    let mut splits = Vec::new();
    let mut consumed = 0;
    for (position, solution) in solutions.iter().enumerate() {
        if position > 0 && boundaries.contains(&consumed) {
            splits.push(position);
        }
        consumed += solution.chars().count();
    }

    if consumed != letters || splits.len() != boundaries.len() {
        return (clue.to_string(), None);
    }

    let clue_text = clue[..capture.get(0).unwrap().start()].trim().to_string();
    (clue_text, Some(splits))
}
