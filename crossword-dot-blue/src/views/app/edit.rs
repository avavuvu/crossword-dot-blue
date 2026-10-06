pub mod clues;
mod feature;
mod form;
mod preview;
mod share;

use boutique::htmx::partial;
use bq_components::{attrs, button};
use crossword_tools::puzzle::Puzzle;
use maud::{Markup, html};

pub use feature::feature_block;
pub use form::save_status;
pub use preview::grid_preview;
pub use share::share_block;

use crate::{
    models::{puzzle, user},
    views::{layouts::{head, shell}, viewer::Viewer},
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

pub fn page(model: &puzzle::Model, author: &user::Model, editor: &user::Model, viewer: &Viewer, puzzle: &Puzzle) -> Markup {
    let heading = model.display_title();
    let key = model.key.as_str();
    let action = model.edit_path();

    shell(
        head(format!("{heading} — Crossword Dot Blue")).htmx().css("edit"),
        viewer,
        html! {
            main.edit {
                section.editing {
                    (form::meta_form(model, author, editor, puzzle))

                    p.actions {
                        (button(html! { "View puzzle" }).href(model.path(&author.username)).secondary().attrs(attrs!(data_leave)))
                        (button(html! { "Back to dashboard" }).href("/app").ghost().attrs(attrs!(data_leave)))
                    }

                    (share_block(model, author))

                    @if editor.is_admin {
                        (feature_block(model))
                    }

                    (clues::clue_list(key, puzzle))

                    details.danger-zone {
                        summary { "Delete this puzzle" }
                        p { "This removes the puzzle and its clues for good. Players lose their saved progress." }
                        (button(html! { "Delete puzzle" }).post(format!("{action}/delete")).danger())
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
