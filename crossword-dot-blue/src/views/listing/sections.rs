use maud::{Markup, html};

use super::cards;
use crate::{models::listing::Entry, views::state::ViewState};

pub fn results(entries: &[Entry], state: &ViewState) -> Markup {
    let entries: Vec<&Entry> = entries.iter().collect();

    html! {
        @if entries.is_empty() {
            p.empty-state { "No puzzles match these filters." }
        } @else {
            (cards(&entries, state))
        }
    }
}
