use boutique::{components::Button, views::base};
use maud::{Markup, html};

use crate::{
    components::ui::{Card, footer, header::header},
    models::{listing::{self, Entry, Featured}, puzzle::Category},
    views::{layouts::page, listing as listing_view, state::ViewState},
};

const PER_CATEGORY: usize = 6;

pub fn index(state: &ViewState, buzzwords: &[&str], featured: &Featured, public: &[Entry]) -> Markup {
    base(
        &page("Crossword dot blue"),
        html! {
            (header(state))
            section.hero { (super::intro::intro(buzzwords)) }
            main.lander.listing {
                h1.sr-only { "Crossword dot blue" }

                p {
                    "Crossword dot blue is a free and open source crossword repository. It has (tentatively) launched on 24 September 2026, with submissions open to the public. "
                    a.link href="/about" {
                        "Find out more here."
                    }
                }

                @if !featured.is_empty() {
                    section.category-section.featured {
                        @for category in [Category::Big, Category::Midi, Category::Mini] {
                            @if let Some(entry) = featured.get(category) {
                                @let card = Card::new(entry).standalone().viewer(state);
                                @if category == Category::Midi {
                                    (card.row_reverse())
                                } @else {
                                    (card.row())
                                }
                            }
                        }
                    }
                }

                @for category in Category::ALL {
                    @let entries = listing::in_category(public, category);
                    @if !entries.is_empty() {
                        section.category-section {
                            header.section-header {
                                h2 { (category.label()) }
                                (Button::link(html! { "See all" }, category.browse_path()).ghost().small())
                            }
                            (listing_view::cards(&entries[..entries.len().min(PER_CATEGORY)], state))
                        }
                    }
                }

                @if public.is_empty() {
                    p.empty-state { "No public puzzles yet." }
                }
            }
            (footer())
        }
    )
}
