use std::collections::BTreeMap;

use crate::puzzle::Meta;

pub const SPLIT_CHARACTER: char = '|';

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("line {line}: text before the first section")]
    TextBeforeSection { line: usize },
    #[error("line {line}: unknown section {name:?}")]
    UnknownSection { line: usize, name: String },
    #[error("line {line}: section {name:?} appears twice")]
    DuplicateSection { line: usize, name: String },
    #[error("missing section {0}")]
    MissingSection(&'static str),
    #[error("line {line}: expected `key: value`")]
    BadMetadata { line: usize },
    #[error("line {line}: bad rebus entry {entry:?}, expected `symbol=word`")]
    BadRebus { line: usize, entry: String },
    #[error("line {line}: difficulty {value:?} is not a number from 0 to 255")]
    BadDifficulty { line: usize, value: String },
    #[error("line {line}: row has {found} cells, expected {expected}")]
    RowWidth { line: usize, expected: usize, found: usize },
    #[error("{section} has {found} rows, expected {expected}")]
    RowCount { section: &'static str, expected: usize, found: usize },
    #[error("line {line}: cannot parse clue, expected `A1. text ~ ANSWER` or `A1 ^key: value`")]
    BadClue { line: usize },
    #[error("line {line}: clue {id} appears twice")]
    DuplicateClue { line: usize, id: String },
    #[error("line {line}: clue {id} answer {answer} does not match the grid {expected}")]
    AnswerMismatch { line: usize, id: String, answer: String, expected: String },
    #[error("line {line}: clue {id} has a split inside a rebus cell")]
    SplitInRebus { line: usize, id: String },
    #[error("line {line}: clue {id} does not start a word in the grid")]
    UnknownClue { line: usize, id: String },
    #[error("clue {0} is missing")]
    MissingClue(String),
    #[error("line {line}: expected `X: style ...`")]
    BadStyleLine { line: usize },
    #[error("line {line}: unknown style {name:?}")]
    UnknownStyle { line: usize, name: String },
    #[error("line {line}: style symbol {symbol:?} is not defined")]
    UnknownStyleSymbol { line: usize, symbol: char },
}

#[derive(Clone, Copy)]
pub struct Line<'a> {
    pub number: usize,
    pub text: &'a str,
}

impl Line<'_> {
    pub fn is_blank(&self) -> bool {
        self.text.trim().is_empty()
    }
}

pub struct Metadata {
    pub meta: Meta,
    pub rebuses: BTreeMap<String, String>,
    pub split: Option<char>,
}

pub struct FileClue<'a> {
    pub line: usize,
    pub body: &'a str,
    pub answer: &'a str,
    pub metadata: BTreeMap<String, String>,
}
