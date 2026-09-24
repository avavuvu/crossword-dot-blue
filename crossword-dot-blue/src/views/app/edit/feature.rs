use boutique::components::Button;
use maud::{Markup, html};

use crate::models::puzzle;

pub fn feature_block(puzzle: &puzzle::Model) -> Markup {
    let feature_url = format!("{}/feature", puzzle.edit_path());

    html! {
        section.feature id="feature" {
            h2 { "Featured" }
            @if let Some(featured_at) = puzzle.featured_at {
                p { "Featured on the front page since " (featured_at.format("%-d %b %Y")) "." }
                (Button::button(html! { "Remove from featured" }).ghost().small()
                    .hx_post(&feature_url)
                    .hx_target("#feature")
                    .hx_swap("outerHTML"))
            } @else if puzzle.is_public {
                p { "Show this puzzle on the front page." }
                (Button::button(html! { "Feature this puzzle" }).secondary().small()
                    .hx_post(&feature_url)
                    .hx_target("#feature")
                    .hx_swap("outerHTML"))
            } @else {
                p { "Only public puzzles can be featured." }
            }
        }
    }
}
