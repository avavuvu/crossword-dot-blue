use crossword_tools::puzzle::Puzzle;
use maud::{Markup, html};

use crate::{
    components::ui::markdown_textarea,
    models::{puzzle::{self, Region}, user},
};

pub fn save_status(puzzle: &puzzle::Model) -> Markup {
    html! {
        @if puzzle.is_public { "Public" } @else { "Private" }
        ", saved at "
        time datetime=(puzzle.updated_at.to_rfc3339()) { (puzzle.updated_at.format("%-d %b %Y, %H:%M")) }
    }
}

fn toggle(name: &str, label: &str, checked: bool) -> Markup {
    html! {
        div.input-component.toggle {
            label {
                input type="checkbox" name=(name) value="1" checked[checked];
                " " (label)
            }
        }
    }
}

pub fn meta_form(puzzle: &puzzle::Model, author: &user::Model, viewer: &user::Model, content: &Puzzle) -> Markup {
    let heading = puzzle.display_title();
    let action = puzzle.edit_path();
    let difficulty = puzzle.difficulty.map(|d| d.to_string()).unwrap_or_default();

    html! {
        form.meta
            method="POST"
            action=(action)
            hx-post=(action)
            hx-trigger="change delay:500ms, submit"
            hx-sync="this:replace"
            hx-target="#save-status"
            data-autosave
        {
            div.input-component.title {
                label for="title" .sr-only { "Title" }
                input.title-input
                    id="title"
                    name="title"
                    type="text"
                    value=(puzzle.title.as_deref().unwrap_or(""))
                    placeholder=(heading)
                    autocomplete="off";
                p.error id="title-error" {}
            }

            p.dimensions {
                (content.width) "×" (content.height)
            }

            div.input-component {
                label for="notes" { "Notes" }
                (markdown_textarea(true, html! {
                    textarea id="notes" name="notes" rows="4" data-markdown {
                        (puzzle.notes.as_deref().unwrap_or(""))
                    }
                }))
                p.error id="notes-error" {}
            }

            @if author.id != viewer.id {
                p.hint { "Editing as admin. This puzzle belongs to @" (author.username) "." }
            }

            (toggle("themed", "Themed", puzzle.themed))
            (toggle("is_cryptic", "Cryptic", puzzle.is_cryptic))

            div.input-component {
                label for="difficulty" { "Difficulty" }
                input id="difficulty" name="difficulty" type="number" min="1" max="5" value=(difficulty);
                p.error id="difficulty-error" {}
            }

            div.input-component {
                label for="region" { "Region" }
                input id="region" name="region" type="text" list="region-options" value=(puzzle.region.as_ref().map(Region::as_str).unwrap_or(""));
                datalist id="region-options" {
                    @for region in Region::DEFAULTS {
                        option value=(region.as_str()) {}
                    }
                }
                p.error id="region-error" {}
            }

            (toggle("is_public", "Public", puzzle.is_public))

            p.save-status id="save-status" { (save_status(puzzle)) }
        }
    }
}
