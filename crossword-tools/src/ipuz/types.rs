use serde::Deserialize;

use super::Error;

#[derive(Deserialize)]
pub struct IpuzFile {
    pub kind: Vec<String>,
    pub dimensions: Dimensions,
    pub puzzle: Vec<Vec<PuzzleCell>>,
    pub solution: Vec<Vec<SolutionCell>>,
    #[serde(default)]
    pub clues: Clues,
    #[serde(default = "default_block")]
    pub block: String,
    #[serde(default = "default_empty")]
    pub empty: Label,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub copyright: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub intro: Option<String>,
    #[serde(default)]
    pub explanation: Option<String>,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub uniqueid: Option<String>,
    #[serde(default)]
    pub difficulty: Option<String>,
}

fn default_block() -> String {
    "#".into()
}

fn default_empty() -> Label {
    Label::Number(0)
}

fn default_null() -> Label {
    Label::Null
}

#[derive(Deserialize)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Label {
    Null,
    Number(u16),
    Text(String),
}

pub enum LabelKind {
    Block,
    Void,
    Empty,
    Numbered(u16),
}

impl Label {
    pub fn kind(&self, block: &str, empty: &Label) -> LabelKind {
        match self {
            Label::Null => LabelKind::Void,
            Label::Number(0) => LabelKind::Empty,
            Label::Number(n) => LabelKind::Numbered(*n),
            Label::Text(text) if text == block => LabelKind::Block,
            Label::Text(text) if matches!(empty, Label::Text(e) if e == text) => LabelKind::Empty,
            Label::Text(text) => match text.parse::<u16>() {
                Ok(0) | Err(_) => LabelKind::Empty,
                Ok(n) => LabelKind::Numbered(n),
            },
        }
    }

    pub fn number(&self) -> Result<u16, Error> {
        match self {
            Label::Number(n) => Ok(*n),
            Label::Text(text) => text.parse().map_err(|_| Error::BadClueNumber(text.clone())),
            Label::Null => Err(Error::BadClueNumber("null".into())),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum PuzzleCell {
    Label(Label),
    Object(PuzzleCellObject),
}

#[derive(Deserialize)]
pub struct PuzzleCellObject {
    #[serde(default = "default_null")]
    pub cell: Label,
    #[serde(default)]
    pub style: Option<Style>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct Style {
    #[serde(default)]
    pub shapebg: Option<String>,
    #[serde(default)]
    pub highlight: Option<bool>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SolutionCell {
    Null,
    Text(String),
    Object {
        #[serde(default)]
        value: Option<String>,
    },
}

#[derive(Deserialize, Default)]
pub struct Clues {
    #[serde(rename = "Across", default)]
    pub across: Vec<ClueEntry>,
    #[serde(rename = "Down", default)]
    pub down: Vec<ClueEntry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ClueEntry {
    Pair(Label, String),
    Object(ClueObject),
}

#[derive(Deserialize)]
pub struct ClueObject {
    pub number: Label,
    #[serde(default)]
    pub clue: Option<String>,
    #[serde(default)]
    pub continued: Vec<ClueRef>,
    #[serde(default)]
    pub references: Vec<ClueRef>,
}

#[derive(Deserialize)]
pub struct ClueRef {
    pub direction: String,
    pub number: Label,
}
