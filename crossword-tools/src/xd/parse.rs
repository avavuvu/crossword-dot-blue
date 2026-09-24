use std::collections::BTreeMap;

use regex::Regex;

use super::types::{Error, FileClue, Line, Metadata};
use crate::grid;
use crate::puzzle::{Cell, CellClues, Clue, Direction, Meta, Puzzle, Style};
use crate::utils::{assign_slot, generate_clue_id, index_clues, slot_solutions, trimmed_or_none};

fn split_sections(text: &str) -> Result<BTreeMap<String, Vec<Line<'_>>>, Error> {
    let mut sections: BTreeMap<String, Vec<Line>> = BTreeMap::new();
    let mut current: Option<String> = None;
    let mut in_comment = false;

    for (index, raw) in text.lines().enumerate() {
        let number = index + 1;
        let trimmed = raw.trim_end();

        if in_comment {
            in_comment = !trimmed.ends_with("-->");
            continue;
        }
        if trimmed.starts_with("<!--") {
            in_comment = !trimmed.ends_with("-->");
            continue;
        }

        if let Some(name) = trimmed.strip_prefix("## ") {
            let name = name.trim().to_lowercase();
            if !matches!(name.as_str(), "metadata" | "grid" | "clues" | "style" | "start" | "notes") {
                return Err(Error::UnknownSection { line: number, name });
            }
            if sections.contains_key(&name) {
                return Err(Error::DuplicateSection { line: number, name });
            }
            sections.insert(name.clone(), Vec::new());
            current = Some(name);
            continue;
        }

        match &current {
            Some(name) => sections.get_mut(name).unwrap().push(Line { number, text: trimmed }),
            None if trimmed.trim().is_empty() => {}
            None => return Err(Error::TextBeforeSection { line: number }),
        }
    }

    Ok(sections)
}

fn parse_metadata(lines: &[Line]) -> Result<Metadata, Error> {
    let mut meta = Meta::default();
    let mut rebuses = BTreeMap::new();
    let mut split = None;

    for &Line { number: line, text } in lines {
        if text.trim().is_empty() {
            continue;
        }
        let (key, value) = text.split_once(':').ok_or(Error::BadMetadata { line })?;
        let key = key.trim().to_lowercase();
        let value = value.trim();

        match key.as_str() {
            "title" => meta.title = trimmed_or_none(value),
            "author" => meta.author = trimmed_or_none(value),
            "date" => meta.date = trimmed_or_none(value),
            "copyright" => meta.copyright = trimmed_or_none(value),
            "description" => meta.description = trimmed_or_none(value),
            "notes" => meta.notes = trimmed_or_none(value),
            "id" => meta.id = trimmed_or_none(value),
            "difficulty" => {
                if !value.is_empty() {
                    let parsed = value
                        .parse()
                        .map_err(|_| Error::BadDifficulty { line, value: value.to_string() })?;
                    meta.difficulty = Some(parsed);
                }
            }
            "rebus" => {
                for entry in value.split_whitespace() {
                    let (symbol, word) = entry
                        .split_once('=')
                        .ok_or_else(|| Error::BadRebus { line, entry: entry.to_string() })?;
                    rebuses.insert(symbol.to_string(), word.to_string());
                }
            }
            "splitcharacter" => split = value.chars().next(),
            _ => {
                if let Some(value) = trimmed_or_none(value) {
                    meta.extra.insert(key, value);
                }
            }
        }
    }

    Ok(Metadata { meta, rebuses, split })
}

fn grid_rows<'a>(lines: &[Line<'a>]) -> Vec<Line<'a>> {
    lines.iter().copied().filter(|line| !line.is_blank()).collect()
}

fn check_width(rows: &[Line], width: usize) -> Result<(), Error> {
    for row in rows {
        let found = row.text.chars().count();
        if found != width {
            return Err(Error::RowWidth { line: row.number, expected: width, found });
        }
    }
    Ok(())
}

fn parse_grid(lines: &[Line], rebuses: &BTreeMap<String, String>) -> Result<(usize, usize, Vec<Cell>), Error> {
    let rows = grid_rows(lines);
    let height = rows.len();
    let width = rows.first().map_or(0, |row| row.text.chars().count());
    check_width(&rows, width)?;

    let mut cells = Vec::with_capacity(width * height);

    for row in rows {
        for symbol in row.text.chars() {
            let cell = match symbol {
                '.' | '#' => Cell::Block { styles: Vec::new() },
                _ => Cell::Letter {
                    solution: rebuses.get(&symbol.to_string()).cloned().unwrap_or_else(|| symbol.to_string()),
                    number: None,
                    clues: CellClues::default(),
                    styles: Vec::new(),
                    prefilled: None,
                },
            };
            cells.push(cell);
        }
    }

    Ok((width, height, cells))
}

fn apply_styles(lines: &[Line], width: usize, height: usize, cells: &mut [Cell]) -> Result<(), Error> {
    let mut symbols: BTreeMap<char, Vec<Style>> = BTreeMap::new();
    let mut rest = lines;

    while let Some((&Line { number: line, text }, tail)) = rest.split_first() {
        rest = tail;
        if text.trim().is_empty() {
            if symbols.is_empty() {
                continue;
            }
            break;
        }

        let (symbol, names) = text.split_once(':').ok_or(Error::BadStyleLine { line })?;
        let mut symbol_chars = symbol.trim().chars();
        let symbol = match (symbol_chars.next(), symbol_chars.next()) {
            (Some(c), None) => c,
            _ => return Err(Error::BadStyleLine { line }),
        };

        let mut styles = Vec::new();
        for name in names.split_whitespace() {
            let style = name
                .parse()
                .map_err(|_| Error::UnknownStyle { line, name: name.to_string() })?;
            styles.push(style);
        }
        styles.sort();
        symbols.insert(symbol, styles);
    }

    let rows = grid_rows(rest);
    if rows.len() != height {
        return Err(Error::RowCount { section: "style", expected: height, found: rows.len() });
    }
    check_width(&rows, width)?;

    for (row, Line { number: line, text }) in rows.into_iter().enumerate() {
        for (col, symbol) in text.chars().enumerate() {
            if symbol == '.' || symbol == '#' {
                continue;
            }
            let styles = symbols
                .get(&symbol)
                .ok_or(Error::UnknownStyleSymbol { line, symbol })?;
            match &mut cells[row * width + col] {
                Cell::Block { styles: target } | Cell::Letter { styles: target, .. } => {
                    *target = styles.clone();
                }
            }
        }
    }

    Ok(())
}

fn apply_start(lines: &[Line], width: usize, height: usize, cells: &mut [Cell]) -> Result<(), Error> {
    let rows = grid_rows(lines);
    if rows.len() != height {
        return Err(Error::RowCount { section: "start", expected: height, found: rows.len() });
    }
    check_width(&rows, width)?;

    for (row, Line { text, .. }) in rows.into_iter().enumerate() {
        for (col, symbol) in text.chars().enumerate() {
            if matches!(symbol, '.' | '#' | ' ' | '_') {
                continue;
            }
            if let Cell::Letter { prefilled, .. } = &mut cells[row * width + col] {
                *prefilled = Some(symbol.to_string());
            }
        }
    }

    Ok(())
}

fn collect_clues<'a>(lines: &[Line<'a>]) -> Result<BTreeMap<String, FileClue<'a>>, Error> {
    let clue_line = Regex::new(r"^([AD]\d+)\.\s*(.*?)\s*~\s*(\S.*?)\s*$").unwrap();
    let clue_meta = Regex::new(r"^([AD]\d+)\s+\^(\w+):\s*(.*?)\s*$").unwrap();
    let mut clues: BTreeMap<String, FileClue> = BTreeMap::new();

    for &Line { number: line, text } in lines {
        if text.trim().is_empty() {
            continue;
        }

        if let Some(capture) = clue_line.captures(text) {
            let id = capture[1].to_string();
            if clues.contains_key(&id) {
                return Err(Error::DuplicateClue { line, id });
            }
            let body = capture.get(2).unwrap().as_str();
            let answer = capture.get(3).unwrap().as_str();
            clues.insert(id, FileClue { line, body, answer, metadata: BTreeMap::new() });
            continue;
        }

        if let Some(capture) = clue_meta.captures(text) {
            let id = &capture[1];
            let clue = clues.get_mut(id).ok_or(Error::BadClue { line })?;
            clue.metadata.insert(capture[2].to_lowercase(), capture[3].to_string());
            continue;
        }

        return Err(Error::BadClue { line });
    }

    Ok(clues)
}

fn answer_and_splits(
    answer: &str,
    split: Option<char>,
    solutions: &[&str],
    line: usize,
    id: &str,
) -> Result<(String, Vec<usize>), Error> {
    let expected: String = solutions.concat();

    let Some(split) = split.filter(|s| answer.contains(*s)) else {
        return check_answer(answer, &expected, line, id).map(|a| (a, Vec::new()));
    };

    let joined: String = answer.chars().filter(|&c| c != split).collect();
    let joined = check_answer(&joined, &expected, line, id)?;

    let mut boundaries = Vec::new();
    let mut letters = 0;
    for segment in answer.split(split) {
        letters += segment.chars().count();
        boundaries.push(letters);
    }
    boundaries.pop();

    let mut splits = Vec::new();
    let mut consumed = 0;
    for (position, solution) in solutions.iter().enumerate() {
        if boundaries.contains(&consumed) && position > 0 {
            splits.push(position);
        }
        consumed += solution.chars().count();
    }

    if splits.len() != boundaries.len() {
        return Err(Error::SplitInRebus { line, id: id.to_string() });
    }

    Ok((joined, splits))
}

fn check_answer(answer: &str, expected: &str, line: usize, id: &str) -> Result<String, Error> {
    if answer.to_uppercase() != expected.to_uppercase() {
        return Err(Error::AnswerMismatch {
            line,
            id: id.to_string(),
            answer: answer.to_string(),
            expected: expected.to_string(),
        });
    }
    Ok(expected.to_string())
}

pub fn parse(text: &str) -> Result<Puzzle, Error> {
    let sections = split_sections(text)?;
    let section = |name: &str| sections.get(name).map(Vec::as_slice);

    let Metadata { mut meta, rebuses, split } =
        parse_metadata(section("metadata").ok_or(Error::MissingSection("Metadata"))?)?;

    let (width, height, mut cells) =
        parse_grid(section("grid").ok_or(Error::MissingSection("Grid"))?, &rebuses)?;

    if let Some(lines) = section("style") {
        apply_styles(lines, width, height, &mut cells)?;
    }
    if let Some(lines) = section("start") {
        apply_start(lines, width, height, &mut cells)?;
    }
    if let Some(lines) = section("notes") {
        let joined: Vec<&str> = lines.iter().map(|line| line.text).collect();
        meta.notes = trimmed_or_none(&joined.join("\n"));
    }

    let mut file_clues = collect_clues(section("clues").ok_or(Error::MissingSection("Clues"))?)?;
    let slots = grid::slots(width, height, |i| cells[i].is_block());

    let mut across = Vec::new();
    let mut down = Vec::new();

    for slot in slots {
        let id = generate_clue_id(slot.direction, slot.number);
        let file_clue = file_clues.remove(&id).ok_or_else(|| Error::MissingClue(id.clone()))?;

        let solutions = slot_solutions(&cells, &slot);
        let (answer, splits) = answer_and_splits(file_clue.answer, split, &solutions, file_clue.line, &id)?;
        assign_slot(&mut cells, &slot, &id);

        let mut metadata = file_clue.metadata;
        let hint = metadata.remove("hint");
        let refs = metadata
            .remove("refs")
            .map(|value| value.split_whitespace().map(str::to_string).collect())
            .unwrap_or_default();

        let clue = Clue {
            id,
            number: slot.number,
            direction: slot.direction,
            body: file_clue.body.to_string(),
            answer,
            indexes: slot.indexes,
            splits,
            hint,
            refs,
            metadata,
        };

        match slot.direction {
            Direction::Across => across.push(clue),
            Direction::Down => down.push(clue),
        }
    }

    if let Some((id, clue)) = file_clues.into_iter().next() {
        return Err(Error::UnknownClue { line: clue.line, id });
    }

    across.extend(down);
    let (clues, clue_order) = index_clues(across);

    let mut puzzle = Puzzle { meta, width, height, cells, rebuses, clues, clue_order };
    puzzle.link_clues();

    Ok(puzzle)
}
