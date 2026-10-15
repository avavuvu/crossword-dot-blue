use bq_components::html;
use maud::Markup;

use crate::{components::Card, models::listing::Entry, views::viewer::Viewer};

pub fn cards(entries: &[&Entry], viewer: &Viewer) -> Markup {
    html! {
        @if entries.is_empty() {
            p.empty-viewer { "No puzzles here yet." }
        } @else {
            ul.puzzle-cards {
                @for entry in entries {
                    li { Card(&entry.model, &entry.author, viewer = viewer); }
                }
            }
        }
    }
}
