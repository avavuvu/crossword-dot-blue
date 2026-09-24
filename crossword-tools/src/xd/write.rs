use crate::{
    puzzle::{Cell, Clue, Direction, Meta, Puzzle, Style},
    utils::generate_clue_id,
};

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

use super::types::SPLIT_CHARACTER;

pub fn write_xd(puzzle: &Puzzle) -> String {
    let (grid, rebus) = xd_grid(puzzle);
    let has_splits = puzzle.clues.values().any(|clue| !clue.splits.is_empty());
    let meta = xd_meta(&puzzle.meta, &rebus, has_splits);
    let clues = xd_clues(puzzle);
    let style = xd_style(puzzle);
    let start = xd_start(puzzle);

    let mut out = String::new();

    out.push_str("## Metadata\n\n");
    out.push_str(&meta);
    out.push_str("\n## Grid\n\n");
    out.push_str(&grid);
    out.push_str("\n\n## Clues\n\n");
    out.push_str(&clues);

    if let Some(style) = style {
        out.push_str("\n## Style\n\n");
        out.push_str(&style);
    }

    if let Some(start) = start {
        out.push_str("\n## Start\n\n");
        out.push_str(&start);
    }

    if let Some(notes) = &puzzle.meta.notes {
        out.push_str("\n## Notes\n\n");
        out.push_str(notes.trim());
        out.push('\n');
    }

    out
}

fn xd_start(puzzle: &Puzzle) -> Option<String> {
    let has_prefilled = puzzle
        .cells
        .iter()
        .any(|cell| matches!(cell, Cell::Letter { prefilled: Some(_), .. }));

    if !has_prefilled {
        return None;
    }

    let mut out = String::new();

    for (i, cell) in puzzle.cells.iter().enumerate() {
        match cell {
            Cell::Letter { prefilled: Some(text), .. } => out.push_str(text),
            _ => out.push('.'),
        }

        if (i + 1) % puzzle.width == 0 {
            out.push('\n');
        }
    }

    Some(out)
}

fn xd_answer(puzzle: &Puzzle, clue: &Clue) -> String {
    if clue.splits.is_empty() {
        return clue.answer.to_uppercase();
    }

    let mut out = String::new();
    for (position, &index) in clue.indexes.iter().enumerate() {
        if position > 0 && clue.splits.contains(&position) {
            out.push(SPLIT_CHARACTER);
        }
        if let Cell::Letter { solution, .. } = &puzzle.cells[index] {
            out.push_str(&solution.to_uppercase());
        }
    }
    out
}

fn preferred_symbol(style: Style) -> char {
    match style {
        Style::Circle => 'O',
        Style::Shaded => 'S',
        Style::Empty => 'E',
    }
}

fn style_symbols(puzzle: &Puzzle) -> BTreeMap<&[Style], char> {
    let used: BTreeSet<&[Style]> = puzzle
        .cells
        .iter()
        .map(Cell::styles)
        .filter(|styles| !styles.is_empty())
        .collect();

    let mut symbols = BTreeMap::new();
    let mut taken = BTreeSet::new();

    for styles in &used {
        if let [single] = styles {
            let symbol = preferred_symbol(*single);
            symbols.insert(*styles, symbol);
            taken.insert(symbol);
        }
    }

    let mut spare = ('A'..='Z').filter(|c| !taken.contains(c));
    for styles in used {
        if !symbols.contains_key(styles) {
            let symbol = spare.next().expect("out of letters for style combinations");
            symbols.insert(styles, symbol);
        }
    }

    symbols
}

fn xd_style(puzzle: &Puzzle) -> Option<String> {
    let symbols = style_symbols(puzzle);

    if symbols.is_empty() {
        return None;
    }

    let mut out = String::new();
    for (styles, symbol) in &symbols {
        let names: Vec<&str> = styles.iter().map(|style| style.name()).collect();
        writeln!(out, "{symbol}: {}", names.join(" ")).unwrap();
    }
    out.push('\n');

    for (i, cell) in puzzle.cells.iter().enumerate() {
        let out_char = match symbols.get(cell.styles()) {
            Some(&symbol) => symbol,
            None if cell.is_block() => '#',
            None => '.',
        };

        out.push(out_char);

        if (i + 1) % puzzle.width == 0 {
            out.push('\n');
        }
    }

    Some(out)
}

fn xd_clues(puzzle: &Puzzle) -> String {
    let mut out = String::new();

    out.push_str(&xd_clue_group(puzzle, Direction::Across));
    out.push('\n');
    out.push_str(&xd_clue_group(puzzle, Direction::Down));

    out
}

fn xd_clue_group(puzzle: &Puzzle, direction: Direction) -> String {
    let mut out = String::new();

    let clues = puzzle
        .clue_order
        .iter()
        .filter_map(|id| puzzle.clues.get(id))
        .filter(|clue| clue.direction == direction);

    for clue in clues {
        let id = generate_clue_id(clue.direction, clue.number);
        let answer = xd_answer(puzzle, clue);

        writeln!(out, "{id}. {} ~ {answer}", clue.body).unwrap();

        if let Some(hint) = &clue.hint {
            writeln!(out, "{id} ^hint: {hint}").unwrap();
        }

        if !clue.refs.is_empty() {
            writeln!(out, "{id} ^refs: {}", clue.refs.join(" ")).unwrap();
        }

        for (key, value) in &clue.metadata {
            writeln!(out, "{id} ^{key}: {value}").unwrap();
        }
    }

    out
}

fn xd_grid(puzzle: &Puzzle) -> (String, BTreeMap<char, String>) {
    let mut out = String::new();
    let mut rebus = BTreeMap::new();
    let mut markers: BTreeMap<&str, char> = BTreeMap::new();

    for word in puzzle.rebuses.values() {
        let marker = rebus_marker(markers.len());
        markers.insert(word, marker);
        rebus.insert(marker, word.clone());
    }

    for (i, cell) in puzzle.cells.iter().enumerate() {
        let out_char = match cell {
            Cell::Block { .. } => ".".to_string(),
            Cell::Letter { solution, .. } => {
                if solution.chars().count() != 1 {
                    let marker = *markers.entry(solution).or_insert_with(|| {
                        let marker = rebus_marker(rebus.len());
                        rebus.insert(marker, solution.clone());
                        marker
                    });
                    marker.to_string()
                } else {
                    solution.clone()
                }
            }
        };

        out.push_str(&out_char);

        if (i + 1) % puzzle.width == 0 {
            out.push('\n');
        }
    }

    (out, rebus)
}

fn rebus_marker(index: usize) -> char {
    const MARKERS: &[u8] = b"123456789abcdefghijklmnopqrstuvwxyz";

    match MARKERS.get(index) {
        Some(&byte) => byte as char,
        None => panic!("xd supports at most {} rebus cells", MARKERS.len()),
    }
}


fn xd_meta(meta: &Meta, rebus: &BTreeMap<char, String>, has_splits: bool) -> String {
    let mut out = String::new();

    let fields = [
        ("title", meta.title.as_deref()),
        ("author", meta.author.as_deref()),
        ("date", meta.date.as_deref()),
        ("copyright", meta.copyright.as_deref()),
        ("description", meta.description.as_deref()),
        ("id", meta.id.as_deref()),
    ];

    for (key, value) in fields {
        if let Some(value) = value {
            writeln!(out, "{key}: {value}").unwrap();
        }
    }

    if let Some(difficulty) = meta.difficulty {
        writeln!(out, "difficulty: {difficulty}").unwrap();
    }

    if !rebus.is_empty() {
        let entries: Vec<String> = rebus
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect();
        writeln!(out, "rebus: {}", entries.join(" ")).unwrap();
    }

    if has_splits {
        writeln!(out, "splitcharacter: {SPLIT_CHARACTER}").unwrap();
    }

    for (key, value) in &meta.extra {
        writeln!(out, "{key}: {value}").unwrap();
    }

    out
}
