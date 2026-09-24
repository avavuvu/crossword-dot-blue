use std::collections::BTreeMap;

use super::types::{ClueEntry, Dimensions, IpuzFile, LabelKind, PuzzleCell, SolutionCell, Style as IpuzStyle};
use crate::grid;
use crate::puzzle::{Cell, CellClues, Clue, Direction, Meta, Puzzle, Style};
use crate::utils::{
    ShapeError, assign_slot, check_shape, find_splits_in_clue, generate_clue_id, index_clues, register_rebus,
    slot_solutions, trimmed_or_none,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("not a crossword, kind is {0:?}")]
    NotCrossword(Vec<String>),
    #[error("{field} {source}")]
    Shape { field: &'static str, source: ShapeError },
    #[error("cell at row {row}, col {col} has no solution")]
    MissingSolution { row: usize, col: usize },
    #[error("cell at row {row}, col {col} is numbered {file} in the file but {derived} by the grid")]
    NumberMismatch { row: usize, col: usize, file: u16, derived: u16 },
    #[error("clue number {0:?} is not a number")]
    BadClueNumber(String),
    #[error("clue {0} is missing from the file")]
    MissingClue(String),
    #[error("clue {0} in the file does not start a word in the grid")]
    UnknownClue(String),
}

struct FileClue {
    body: String,
    refs: Vec<String>,
}

fn check_rows<T>(field: &'static str, rows: &[Vec<T>], width: usize, height: usize) -> Result<(), Error> {
    check_shape(rows, width, height, Vec::len).map_err(|source| Error::Shape { field, source })
}

fn collect_clues(
    entries: Vec<ClueEntry>,
    direction: Direction,
    into: &mut BTreeMap<String, FileClue>,
) -> Result<(), Error> {
    for entry in entries {
        let (number, body, refs) = match entry {
            ClueEntry::Pair(number, body) => (number, body, Vec::new()),
            ClueEntry::Object(object) => {
                let mut refs = Vec::new();
                for r in object.continued.into_iter().chain(object.references) {
                    let dir = if r.direction.eq_ignore_ascii_case("down") {
                        Direction::Down
                    } else {
                        Direction::Across
                    };
                    refs.push(generate_clue_id(dir, r.number.number()?));
                }
                (object.number, object.clue.unwrap_or_default(), refs)
            }
        };
        let id = generate_clue_id(direction, number.number()?);
        into.insert(id, FileClue { body, refs });
    }
    Ok(())
}

pub fn parse(text: &str) -> Result<Puzzle, Error> {
    let file: IpuzFile = serde_json::from_str(text)?;

    if !file.kind.iter().any(|k| k.contains("crossword")) {
        return Err(Error::NotCrossword(file.kind));
    }

    let Dimensions { width, height } = file.dimensions;
    check_rows("puzzle", &file.puzzle, width, height)?;
    check_rows("solution", &file.solution, width, height)?;

    let mut cells = Vec::with_capacity(width * height);
    let mut file_numbers = Vec::with_capacity(width * height);
    let mut rebuses: BTreeMap<String, String> = BTreeMap::new();

    for (row, (puzzle_row, solution_row)) in file.puzzle.into_iter().zip(file.solution).enumerate() {
        for (col, (puzzle_cell, solution_cell)) in puzzle_row.into_iter().zip(solution_row).enumerate() {
            let (label, style, prefilled) = match puzzle_cell {
                PuzzleCell::Label(label) => (label, IpuzStyle::default(), None),
                PuzzleCell::Object(object) => {
                    (object.cell, object.style.unwrap_or_default(), object.value)
                }
            };

            let solution = match solution_cell {
                SolutionCell::Null => None,
                SolutionCell::Text(text) if text == file.block => None,
                SolutionCell::Text(text) => Some(text),
                SolutionCell::Object { value } => value,
            };

            let number = match label.kind(&file.block, &file.empty) {
                LabelKind::Block => {
                    cells.push(Cell::Block { styles: Vec::new() });
                    file_numbers.push(None);
                    continue;
                }
                LabelKind::Void => {
                    cells.push(Cell::Block { styles: vec![Style::Empty] });
                    file_numbers.push(None);
                    continue;
                }
                LabelKind::Empty => None,
                LabelKind::Numbered(n) => Some(n),
            };

            let Some(solution) = solution else {
                if number.is_none() && prefilled.is_none() {
                    cells.push(Cell::Block { styles: Vec::new() });
                    file_numbers.push(None);
                    continue;
                }
                return Err(Error::MissingSolution { row, col });
            };

            let mut styles = Vec::new();
            if style.shapebg.as_deref() == Some("circle") {
                styles.push(Style::Circle);
            }
            if style.highlight == Some(true) || style.color.is_some() {
                styles.push(Style::Shaded);
            }

            register_rebus(&mut rebuses, &solution);

            cells.push(Cell::Letter {
                solution,
                number: None,
                clues: CellClues::default(),
                styles,
                prefilled: prefilled.as_deref().and_then(trimmed_or_none),
            });
            file_numbers.push(number);
        }
    }

    let slots = grid::slots(width, height, |i| cells[i].is_block());

    for slot in &slots {
        let start = slot.indexes[0];
        if let Some(file_number) = file_numbers[start]
            && file_number != slot.number
        {
            return Err(Error::NumberMismatch {
                row: start / width,
                col: start % width,
                file: file_number,
                derived: slot.number,
            });
        }
    }

    let mut file_clues = BTreeMap::new();
    collect_clues(file.clues.across, Direction::Across, &mut file_clues)?;
    collect_clues(file.clues.down, Direction::Down, &mut file_clues)?;

    let mut across = Vec::new();
    let mut down = Vec::new();

    for slot in slots {
        let id = generate_clue_id(slot.direction, slot.number);
        let FileClue { body, refs } = file_clues.remove(&id).ok_or_else(|| Error::MissingClue(id.clone()))?;
        let (body, splits) = find_splits_in_clue(&body, &slot_solutions(&cells, &slot));
        let answer = assign_slot(&mut cells, &slot, &id);

        let clue = Clue {
            id,
            number: slot.number,
            direction: slot.direction,
            body,
            answer,
            indexes: slot.indexes,
            splits: splits.unwrap_or_default(),
            hint: None,
            refs,
            metadata: BTreeMap::new(),
        };

        match slot.direction {
            Direction::Across => across.push(clue),
            Direction::Down => down.push(clue),
        }
    }

    if let Some(id) = file_clues.into_keys().next() {
        return Err(Error::UnknownClue(id));
    }

    let mut extra = BTreeMap::new();
    for (key, value) in [
        ("intro", file.intro),
        ("explanation", file.explanation),
        ("publisher", file.publisher),
        ("url", file.url),
        ("difficulty", file.difficulty),
    ] {
        if let Some(value) = value.as_deref().and_then(trimmed_or_none) {
            extra.insert(key.to_string(), value);
        }
    }

    let text = |value: Option<String>| value.as_deref().and_then(trimmed_or_none);

    let meta = Meta {
        title: text(file.title),
        author: text(file.author),
        date: text(file.date),
        copyright: text(file.copyright),
        description: None,
        notes: text(file.notes),
        id: text(file.uniqueid),
        difficulty: None,
        extra,
    };

    across.extend(down);
    let (clues, clue_order) = index_clues(across);

    let mut puzzle = Puzzle { meta, width, height, cells, rebuses, clues, clue_order };
    puzzle.link_clues();

    Ok(puzzle)
}
