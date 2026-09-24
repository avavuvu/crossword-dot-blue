use maud::{Markup, html};

use crate::{
    models::{listing::{Entry, Filter}, puzzle::{Category, Region}},
    views::{layouts::{page, shell}, listing, state::ViewState},
};

pub fn index(
    state: &ViewState,
    category: Option<Category>,
    filter: &Filter,
    regions: &[Region],
    entries: &[Entry],
) -> Markup {
    let heading = category.map(Category::label).unwrap_or("All puzzles");
    let action = category.map(|c| c.browse_path()).unwrap_or_else(|| "/browse".to_string());

    shell(
        page(format!("Browse {heading} — Crossword Dot Blue")),
        state,
        html! {
            main.browse.listing {
                header.listing-header {
                    h1 { "Browse" }
                    (listing::category_tabs("/browse", category, filter))
                }
                (listing::filter_form(&action, filter, regions))
                (listing::results(entries, state))
            }
        }
    )
}
