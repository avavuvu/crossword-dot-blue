use maud::{Markup, html};

use crate::{
    models::{category::Category, listing::{Entry, Filter}, region::Region},
    views::{layouts::{HeadExt, head, shell}, listing, viewer::Viewer},
};

pub fn page(
    viewer: &Viewer,
    category: Option<Category>,
    filter: &Filter,
    regions: &[Region],
    entries: &[Entry],
) -> Markup {
    let heading = category.map(Category::label).unwrap_or("All puzzles");
    let action = category.map(|category| category.browse_path()).unwrap_or_else(|| "/browse".to_string());

    shell(
        head(format!("Browse {heading} — Crossword Dot Blue")).css("listing"),
        viewer,
        html! {
            main.browse.listing {
                header.listing-header {
                    h1 { "Browse" }
                    (listing::category_tabs("/browse", category, filter))
                }
                (listing::filter_form(&action, filter, regions))
                (listing::results(entries, viewer))
            }
        }
    )
}
