use maud::Render;
use serde::Serialize;
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "../../crossword-tools/bindings/")]
pub enum UiAction {
    Start,
    Dismiss,
    Reset,
    Hint,
    PrevClue,
    NextClue,
    ToggleDirection,
    ToggleRebus,
    CheckCell,
    CheckWord,
    CheckPuzzle,
    RevealCell,
    RevealWord,
    RevealPuzzle,
}

impl UiAction {
    pub fn as_str(self) -> &'static str {
        match self {
            UiAction::Start => "start",
            UiAction::Dismiss => "dismiss",
            UiAction::Reset => "reset",
            UiAction::Hint => "hint",
            UiAction::PrevClue => "prev-clue",
            UiAction::NextClue => "next-clue",
            UiAction::ToggleDirection => "toggle-direction",
            UiAction::ToggleRebus => "toggle-rebus",
            UiAction::CheckCell => "check-cell",
            UiAction::CheckWord => "check-word",
            UiAction::CheckPuzzle => "check-puzzle",
            UiAction::RevealCell => "reveal-cell",
            UiAction::RevealWord => "reveal-word",
            UiAction::RevealPuzzle => "reveal-puzzle",
        }
    }
}

impl Render for UiAction {
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str(self.as_str());
    }
}

pub fn actions(list: &[UiAction]) -> String {
    list.iter().map(|a| a.as_str()).collect::<Vec<_>>().join(" ")
}
