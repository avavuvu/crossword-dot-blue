mod action;

use bq_components::Button;
use bq_components::{component, setup};
use crossword_tools::puzzle::{Cell, Clue, Direction, Puzzle};
use maud::{Markup, PreEscaped, html};

use action::{UiAction, actions};

use crate::{
    components::{Icon, grid::{self, Fill}, options_panel},
    models::{category::Category, puzzle::{self, get_category}},
    theme::Theme,
    views::markdown,
};

setup!(Crossword);

#[component]
pub fn crossword(
    #[builder(start_fn)]
    model: &puzzle::Model,
    #[builder(start_fn)]
    puzzle: &Puzzle,
    tagline: Option<Markup>,
    options: Option<Theme>,
    ) -> Markup {
    html! {
        cw-crossword key=(model.key) style=(format!("--cols: {}; --rows: {}", puzzle.width, puzzle.height)) bq-setup=(Crossword) {
            script type="application/json" bq-ref="puzzle-json" { (json_script(puzzle)) }

            section.info {
                time datetime="PT0S" bq-ref="timer" { "0:00" }
                p data-entry-preview bq-ref="entry-preview" {}
            }

            (assist_bar(puzzle, options.is_some()))
            (clue_banner())
            (keyboard(puzzle))

            section.board {
                div.stage {
                    div.frame tabindex="0" bq-ref="board" {
                        (grid::grid_svg(puzzle, Fill::Empty))
                    }
                }
                @if let Some(tagline) = tagline {
                    div.tagline {
                        (tagline)
                    }
                }
            }

            (clue_group(puzzle, Direction::Across, "Across"))
            (clue_group(puzzle, Direction::Down, "Down"))

            (intro(model, puzzle))
            (won())
            (almost())
            @if let Some(theme) = options {
                (options_screen(theme))
            }
        }
    }
}

fn options_screen(theme: Theme) -> Markup {
    html! {
        section.options hidden aria-labelledby="options-title" bq-ref="options" {
            h2 id="options-title" tabindex="-1" { "Options" }
            (options_panel(theme))
            div.actions {
                (Button::button(html! { "Back to puzzle" }).action(UiAction::Dismiss.as_str()).primary())
            }
        }
    }
}

fn json_script(puzzle: &Puzzle) -> Markup {
    let json = serde_json::to_string(puzzle).unwrap_or_default();
    PreEscaped(json.replace('<', "\\u003c"))
}

fn intro(model: &puzzle::Model, puzzle: &Puzzle) -> Markup {
    html! {
        section.intro bq-ref="intro" {
            @match get_category(puzzle.width, puzzle.height) {
                Category::Mini => (Icon::CrosswordMini),
                Category::Midi => (Icon::CrosswordMidi),
                Category::Big => (Icon::CrosswordBig),
            }
            p.dimensions {
                (puzzle.width) "×" (puzzle.height)
                @if model.is_cryptic { " · Cryptic" }
            }

            @if let Some(notes) = model.notes.as_deref() {
                div.notes { (markdown::block(notes)) }
            }
            (Button::button(html! { "Let's play!" }).primary().action(UiAction::Start.as_str()))
        }
    }
}

fn won() -> Markup {
    html! {
        section.completion.won hidden data-completion="won" aria-labelledby="won-title" role="status" bq-ref="completion" {
            h2 id="won-title" tabindex="-1" { "Solved in " time bq-ref="completion-time" {} "!" }
            (Icon::CrosswordSolved)
            div.actions {
                (Button::button(html! { "View Puzzle" }).action(UiAction::Dismiss.as_str()).primary())
                (Button::button(html! { "Play again" }).action(UiAction::Reset.as_str()).ghost())
            }
        }
    }
}

fn almost() -> Markup {
    html! {
        section.completion.almost hidden data-completion="complete-but-wrong" aria-labelledby="almost-title" role="status" bq-ref="completion" {
            h2 id="almost-title" tabindex="-1" { "Almost" }
            p.message { "The grid is full, but something is not right yet." }
            div.actions {
                (Button::button(html! { "Keep going" }).action(UiAction::Dismiss.as_str()).primary())
                (Button::button(html! { "Check" }).action(&actions(&[UiAction::CheckPuzzle, UiAction::Dismiss])).ghost())
            }
        }
    }
}

const KEY_ROWS: [&str; 3] = ["QWERTYUIOP", "ASDFGHJKL", "ZXCVBNM"];

fn keyboard(puzzle: &Puzzle) -> Markup {
    let has_rebus = has_rebus(puzzle);

    html! {
        section.keyboard aria-label="Keyboard" bq-ref="keyboard" {
            @for (row, letters) in KEY_ROWS.iter().enumerate() {
                div.row {
                    @if row == 2 {
                        @if has_rebus {
                            button.key.special type="button" data-key="Rebus" aria-pressed="false" bq-ref="rebus-key" { "Rebus" }
                        } @else {
                            button.key.special type="button" data-key="Direction" aria-label="Switch direction" {
                                (Icon::ArrowLeftRight)
                            }
                        }
                    }
                    @for letter in letters.chars() {
                        button.key type="button" data-key=(letter) { (letter) }
                    }
                    @if row == 2 {
                        button.key.special type="button" data-key="Backspace" aria-label="Backspace" {
                            (Icon::Delete)
                        }
                    }
                }
            }
        }
    }
}

fn has_rebus(puzzle: &Puzzle) -> bool {
    puzzle
        .cells
        .iter()
        .any(|cell| matches!(cell, Cell::Letter { solution, .. } if solution.chars().count() > 1))
}

fn clue_banner() -> Markup {
    html! {
        section.clue-banner {
            button.prev type="button" data-action=(UiAction::PrevClue) aria-label="Previous clue" { "‹" }
            button.current-clue type="button" data-action=(UiAction::ToggleDirection) aria-label="Switch direction" {
                span.clue-label bq-ref="clue-label" {}
                " "
                span.clue-text bq-ref="clue-text" {}
            }
            button.next type="button" data-action=(UiAction::NextClue) aria-label="Next clue" { "›" }
        }
    }
}

fn assist_bar(puzzle: &Puzzle, has_options: bool) -> Markup {
    let has_hints = puzzle.clues.values().any(|clue| clue.hint.is_some());
    let has_rebus = has_rebus(puzzle);

    let check_items = || html! {
        button type="button" role="menuitem" data-action=(UiAction::CheckCell) { "Check cell" }
        button type="button" role="menuitem" data-action=(UiAction::CheckWord) { "Check word" }
        button type="button" role="menuitem" data-action=(UiAction::CheckPuzzle) { "Check puzzle" }
    };
    let reveal_items = || html! {
        button type="button" role="menuitem" data-action=(UiAction::RevealCell) { "Reveal cell" }
        button type="button" role="menuitem" data-action=(UiAction::RevealWord) { "Reveal word" }
        button type="button" role="menuitem" data-action=(UiAction::RevealPuzzle) { "Reveal puzzle" }
    };
    let reset_items = || html! {
        button type="button" role="menuitem" data-action=(UiAction::Reset) { "Reset puzzle" }
    };

    html! {
        section.assist {
            (dropdown("check-menu", "Check", "split", check_items()))
            (dropdown("reveal-menu", "Reveal", "split", reveal_items()))
            (dropdown("reset-menu", "Reset", "split", reset_items()))

            (dropdown("tools-menu", "Tools", "grouped", html! {
                (check_items())
                hr;
                (reveal_items())
                hr;
                (reset_items())
            }))

            @if has_hints {
                button.hint type="button" data-action=(UiAction::Hint) hidden bq-ref="hint-button" { "Hint" }
            }

            @if has_rebus {
                button.rebus type="button" data-action=(UiAction::ToggleRebus) aria-pressed="false" bq-ref="rebus" { "Rebus" }
            }

            @if has_options {
                button.options-button type="button" data-action=(UiAction::Options) aria-label="Options" title="Options" { "O" }
            }
        }
    }
}

fn dropdown(id: &str, label: &str, class: &str, items: Markup) -> Markup {
    let anchor = format!("--{id}");

    html! {
        div.dropdown.(class) {
            button type="button" popovertarget=(id) aria-haspopup="menu" style=(format!("anchor-name: {anchor}")) { (label) }
            div.menu id=(id) popover role="menu" style=(format!("position-anchor: {anchor}")) bq-ref="menu" { (items) }
        }
    }
}

fn clue_group(puzzle: &Puzzle, direction: Direction, heading: &str) -> Markup {
    let clues = puzzle
        .clue_order
        .iter()
        .filter_map(|id| puzzle.clues.get(id))
        .filter(|clue| clue.direction == direction);

    html! {
        h2
            .(format!("{direction}-header"))
            .clue-header {
            (heading)
        }

        section
            .clue-group
            .(format!("{direction}-clues")) {

            ol {
                @for clue in clues {
                    (clue_item(clue))
                }
            }
        }
    }
}

fn clue_item(clue: &Clue) -> Markup {
    html! {
        li.clue id={ "clue-" (clue.id) } value=(clue.number) {
            button type="button" data-clue=(clue.id) bq-ref="clue" {
                span.number { (clue.number) }
                span.body { (markdown::inline(&clue.body)) }
            }
        }
    }
}
