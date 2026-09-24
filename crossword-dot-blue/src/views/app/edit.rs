pub mod clues;
mod feature;
mod form;
mod preview;
mod share;

use boutique::{components::Button, htmx::partial};
use crossword_tools::puzzle::Puzzle;
use maud::{Markup, html};

pub use feature::feature_block;
pub use form::save_status;
pub use preview::grid_preview;
pub use share::share_block;

use crate::{
    models::{puzzle, user},
    views::{layouts::{page, shell}, state::ViewState},
};

pub fn save_response(puzzle: &puzzle::Model, author: &user::Model, viewer: &user::Model) -> Markup {
    html! {
        (save_status(puzzle))
        (partial("#share", share_block(puzzle, author)))
        @if viewer.is_admin {
            (partial("#feature", feature_block(puzzle)))
        }
    }
}

pub fn edit(puzzle: &puzzle::Model, author: &user::Model, viewer: &user::Model, content: &Puzzle) -> Markup {
    let heading = puzzle.display_title();
    let key = puzzle.key();
    let action = puzzle.edit_path();

    shell(
        page(format!("{heading} — Crossword Dot Blue")).htmx().module("/assets/editor.js"),
        &ViewState::from(viewer),
        html! {
            main.edit {
                section.editing {
                    (form::meta_form(puzzle, author, viewer, content))

                    p.actions {
                        (Button::link(html! { "View puzzle" }, puzzle.path(&author.username)).secondary().attr("data-leave", ""))
                        (Button::link(html! { "Back to dashboard" }, "/app").ghost().attr("data-leave", ""))
                    }

                    (share_block(puzzle, author))

                    @if viewer.is_admin {
                        (feature_block(puzzle))
                    }

                    (clues::clue_list(key, content))

                    details.danger-zone {
                        summary { "Delete this puzzle" }
                        p { "This removes the puzzle and its clues for good. Players lose their saved progress." }
                        (Button::post(html! { "Delete puzzle" }, format!("{action}/delete")).danger())
                    }
                }

                section.grid {
                    p.disclaimer {
                        "Crossword.blue is not designed for constructing crosswords. It offers basic editing so features like word boundaries can be set when the source file has none."
                    }

                    (preview::preview_tabs())
                    (grid_preview(content, false))
                }
            }
        }
    )
}
