use maud::{Markup, html};

use crate::{components::card, models::listing::Entry, views::viewer::Viewer};

pub fn cards(entries: &[&Entry], viewer: &Viewer) -> Markup {
    html! {
        @if entries.is_empty() {
            p.empty-viewer { "No puzzles here yet." }
        } @else {
            ul.puzzle-cards {
                @for entry in entries {
                    li { (card(&entry.model, &entry.author).viewer(viewer)) }
                }
            }
        }
    }
}
