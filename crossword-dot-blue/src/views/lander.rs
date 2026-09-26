mod intro;

use boutique::{components::Button, views::base};
use maud::{Markup, html};

use crate::{
    components::{Layout, card, footer, header},
    models::{listing::{self, Entry, Featured}, puzzle::Category},
    views::{layouts::{HeadExt, head}, listing as listing_view, viewer::Viewer},
};

const PER_CATEGORY: usize = 6;

pub fn page(viewer: &Viewer, buzzwords: &[&str], featured: &Featured, public: &[Entry]) -> Markup {
    base(
        &head("Crossword dot blue").css("lander").css("listing"),
        html! {
            (header(viewer))
            section.hero { (intro::intro(buzzwords)) }
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
                        ul.featured-cards {
                            @for category in [Category::Big, Category::Midi, Category::Mini] {
                                @if let Some(entry) = featured.get(category) {
                                    @let layout = if category == Category::Midi { Layout::RowReverse } else { Layout::Row };
                                    li { (card(&entry.model, &entry.author).viewer(viewer).layout(layout).tinted(true)) }
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
                            (listing_view::cards(&entries[..entries.len().min(PER_CATEGORY)], viewer))
                        }
                    }
                }

                @if public.is_empty() {
                    p.empty-viewer { "No public puzzles yet." }
                }
            }
            (footer())
        }
    )
}
