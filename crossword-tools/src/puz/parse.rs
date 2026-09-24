use std::collections::{BTreeMap, HashMap};

use puz_parse::PuzError;

use crate::grid;
use crate::puzzle::{Cell, CellClues, Clue, Direction, Meta, Puzzle, Style};
use crate::utils::{
    ShapeError, assign_slot, check_shape, find_splits_in_clue, generate_clue_id, index_clues, register_rebus,
    slot_solutions, trimmed_or_none,
};

const BLOCK: char = '.';
const FREE: char = '-';

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid puz file: {0}")]
    Puz(#[from] PuzError),
    #[error("the solution is scrambled and cannot be read")]
    Scrambled,
    #[error("{field} {source}")]
    Shape { field: &'static str, source: ShapeError },
    #[error("rebus key {key} at row {row}, col {col} is not in the rebus table")]
    UnknownRebus { row: usize, col: usize, key: u8 },
    #[error("clue {0} is missing from the file")]
    MissingClue(String),
    #[error("clue {0} in the file does not start a word in the grid")]
    UnknownClue(String),
}

fn check_rows(field: &'static str, rows: &[String], width: usize, height: usize) -> Result<(), Error> {
    check_shape(rows, width, height, |line| line.chars().count())
        .map_err(|source| Error::Shape { field, source })
}

fn flatten<T: Copy>(grid: Option<&[Vec<T>]>, len: usize, default: T) -> Vec<T> {
    match grid {
        Some(rows) => rows.iter().flatten().copied().collect(),
        None => vec![default; len],
    }
}

pub fn parse(bytes: &[u8]) -> Result<Puzzle, Error> {
    let file = puz_parse::parse_bytes(bytes)?;

    if file.info.is_scrambled {
        return Err(Error::Scrambled);
    }

    let width = file.info.width as usize;
    let height = file.info.height as usize;
    let len = width * height;
    check_rows("solution", &file.grid.solution, width, height)?;
    check_rows("blank", &file.grid.blank, width, height)?;

    let solution: Vec<char> = file.grid.solution.iter().flat_map(|line| line.chars()).collect();
    let blank: Vec<char> = file.grid.blank.iter().flat_map(|line| line.chars()).collect();
    let circles = flatten(file.extensions.circles.as_deref(), len, false);
    let given = flatten(file.extensions.given.as_deref(), len, false);

    let rebus = file.extensions.rebus.as_ref();
    let rebus_keys = flatten(rebus.map(|r| r.grid.as_slice()), len, 0u8);
    let empty_table = HashMap::new();
    let rebus_table = rebus.map_or(&empty_table, |r| &r.table);

    let mut cells = Vec::with_capacity(len);
    let mut rebuses: BTreeMap<String, String> = BTreeMap::new();

    for (index, &letter) in solution.iter().enumerate() {
        if letter == BLOCK {
            cells.push(Cell::Block { styles: Vec::new() });
            continue;
        }

        let solution = match rebus_keys[index] {
            0 => letter.to_string(),
            key => {
                let key = key - 1;
                let word = rebus_table.get(&key).ok_or(Error::UnknownRebus {
                    row: index / width,
                    col: index % width,
                    key,
                })?;
                register_rebus(&mut rebuses, word);
                word.clone()
            }
        };

        let prefilled = (given[index] && blank[index] != FREE && blank[index] != BLOCK)
            .then(|| blank[index].to_string());

        cells.push(Cell::Letter {
            solution,
            number: None,
            clues: CellClues::default(),
            styles: if circles[index] { vec![Style::Circle] } else { Vec::new() },
            prefilled,
        });
    }

    let slots = grid::slots(width, height, |i| cells[i].is_block());

    let mut file_clues: BTreeMap<String, String> = BTreeMap::new();
    for (number, body) in &file.clues.across {
        file_clues.insert(generate_clue_id(Direction::Across, *number), body.clone());
    }
    for (number, body) in &file.clues.down {
        file_clues.insert(generate_clue_id(Direction::Down, *number), body.clone());
    }

    let mut across = Vec::new();
    let mut down = Vec::new();

    for slot in slots {
        let id = generate_clue_id(slot.direction, slot.number);
        let body = file_clues.remove(&id).ok_or_else(|| Error::MissingClue(id.clone()))?;
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
            refs: Vec::new(),
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

    let meta = Meta {
        title: trimmed_or_none(&file.info.title),
        author: trimmed_or_none(&file.info.author),
        copyright: trimmed_or_none(&file.info.copyright),
        notes: trimmed_or_none(&file.info.notes),
        ..Meta::default()
    };

    across.extend(down);
    let (clues, clue_order) = index_clues(across);

    let mut puzzle = Puzzle { meta, width, height, cells, rebuses, clues, clue_order };
    puzzle.link_clues();

    Ok(puzzle)
}
