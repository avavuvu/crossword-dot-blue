use maud::{Markup, html};

use crate::{components::ui::Card, models::listing::Entry, views::state::ViewState};

pub fn cards(entries: &[&Entry], state: &ViewState) -> Markup {
    html! {
        @if entries.is_empty() {
            p.empty-state { "No puzzles here yet." }
        } @else {
            ul.puzzle-cards {
                @for entry in entries {
                    (Card::new(entry).viewer(state))
                }
            }
        }
    }
}
