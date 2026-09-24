use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Puzzle {
    pub meta: Meta,
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub rebuses: BTreeMap<String, String>,
    pub clues: BTreeMap<String, Clue>,
    pub clue_order: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Meta {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub copyright: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub id: Option<String>,
    pub difficulty: Option<u8>,
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(export)]
pub enum Cell {
    Block {
        styles: Vec<Style>,
    },
    #[serde(rename_all = "camelCase")]
    Letter {
        solution: String,
        number: Option<u16>,
        clues: CellClues,
        styles: Vec<Style>,
        prefilled: Option<String>,
    },
}

impl Cell {
    pub fn is_block(&self) -> bool {
        matches!(self, Cell::Block { .. })
    }

    pub fn styles(&self) -> &[Style] {
        match self {
            Cell::Block { styles } | Cell::Letter { styles, .. } => styles,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export)]
pub enum Style {
    Circle,
    Shaded,
    Empty,
}

impl Style {
    pub const ALL: [Style; 3] = [Style::Circle, Style::Shaded, Style::Empty];

    pub fn name(self) -> &'static str {
        match self {
            Style::Circle => "circle",
            Style::Shaded => "shaded",
            Style::Empty => "empty",
        }
    }
}

impl std::str::FromStr for Style {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Style::ALL
            .into_iter()
            .find(|style| style.name() == text)
            .ok_or_else(|| format!("unknown style {text:?}"))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CellClues {
    pub across: Option<String>,
    pub down: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum Direction {
    Across,
    Down,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Direction::Across => "across",
                Direction::Down => "down",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Clue {
    pub id: String,
    pub number: u16,
    pub direction: Direction,
    pub body: String,
    pub answer: String,
    pub indexes: Vec<usize>,
    pub splits: Vec<usize>,
    pub hint: Option<String>,
    pub refs: Vec<String>,
    pub metadata: BTreeMap<String, String>,
}
