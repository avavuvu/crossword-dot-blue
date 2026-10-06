use bq_components::button;
use maud::{Markup, html};

use crate::models::puzzle;

pub fn feature_block(model: &puzzle::Model) -> Markup {
    let feature_url = format!("{}/feature", model.edit_path());

    html! {
        section.feature id="feature" {
            h2 { "Featured" }
            @if let Some(featured_at) = model.featured_at {
                p { "Featured on the front page since " (featured_at.format("%-d %b %Y")) "." }
                (button(html! { "Remove from featured" }).ghost().small()
                    .hx_post(&feature_url)
                    .hx_target("#feature")
                    .hx_swap("outerHTML"))
            } @else if model.is_public {
                p { "Show this puzzle on the front page." }
                (button(html! { "Feature this puzzle" }).secondary().small()
                    .hx_post(&feature_url)
                    .hx_target("#feature")
                    .hx_swap("outerHTML"))
            } @else {
                p { "Only public puzzles can be featured." }
            }
        }
    }
}
