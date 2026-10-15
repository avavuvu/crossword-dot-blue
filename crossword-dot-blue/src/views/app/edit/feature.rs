use bq_components::{Button, html};
use maud::Markup;

use crate::models::puzzle;

pub fn feature_block(model: &puzzle::Model) -> Markup {
    let feature_url = format!("{}/feature", model.edit_path());

    html! {
        section.feature id="feature" {
            h2 { "Featured" }
            @if let Some(featured_at) = model.featured_at {
                p { "Featured on the front page since " (featured_at.format("%-d %b %Y")) "." }
                Button .ghost .small hx-post=(feature_url) hx-target="#feature" hx-swap="outerHTML" { "Remove from featured" }
            } @else if model.is_public {
                p { "Show this puzzle on the front page." }
                Button .secondary .small hx-post=(feature_url) hx-target="#feature" hx-swap="outerHTML" { "Feature this puzzle" }
            } @else {
                p { "Only public puzzles can be featured." }
            }
        }
    }
}
