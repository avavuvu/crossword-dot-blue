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
    views::{layouts::{HeadExt, head, shell}, viewer::Viewer},
};

pub fn save_response(model: &puzzle::Model, author: &user::Model, viewer: &user::Model) -> Markup {
    html! {
        (save_status(model))
        (partial("#share", share_block(model, author)))
        @if viewer.is_admin {
            (partial("#feature", feature_block(model)))
        }
    }
}

pub fn page(model: &puzzle::Model, author: &user::Model, viewer: &user::Model, puzzle: &Puzzle) -> Markup {
    let heading = model.display_title();
    let key = model.key();
    let action = model.edit_path();

    shell(
        head(format!("{heading} — Crossword Dot Blue")).htmx().entry("editor").css("edit"),
        &Viewer::from(viewer),
        html! {
            main.edit {
                section.editing {
                    (form::meta_form(model, author, viewer, puzzle))

                    p.actions {
                        (Button::link(html! { "View puzzle" }, model.path(&author.username)).secondary().attr("data-leave", ""))
                        (Button::link(html! { "Back to dashboard" }, "/app").ghost().attr("data-leave", ""))
                    }

                    (share_block(model, author))

                    @if viewer.is_admin {
                        (feature_block(model))
                    }

                    (clues::clue_list(key, puzzle))

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
                    (grid_preview(puzzle, false))
                }
            }
        }
    )
}
