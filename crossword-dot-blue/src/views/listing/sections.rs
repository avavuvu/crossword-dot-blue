use maud::{Markup, html};

use super::cards;
use crate::{models::listing::Entry, views::viewer::Viewer};

pub fn results(entries: &[Entry], viewer: &Viewer) -> Markup {
    let entries: Vec<&Entry> = entries.iter().collect();

    html! {
        @if entries.is_empty() {
            p.empty-viewer { "No puzzles match these filters." }
        } @else {
            (cards(&entries, viewer))
        }
    }
}
