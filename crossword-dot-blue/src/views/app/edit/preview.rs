use boutique::htmx::partial;
use crossword_tools::{puzzle::Puzzle, xd};
use maud::{Markup, html};

use crate::views::grid::{self, Fill};

const VIEWS: [(&str, &str); 3] = [("grid", "Grid"), ("json", "JSON"), ("xd", ".xd")];

fn maybe_partial(as_partial: bool, target: &str, content: Markup) -> Markup {
    if as_partial { partial(target, content) } else { content }
}

pub fn preview_tabs() -> Markup {
    html! {
        div.preview-tabs role="group" aria-label="Preview" {
            @for (index, (value, label)) in VIEWS.iter().enumerate() {
                label.tab {
                    input type="radio" name="preview-view" value=(value) checked[index == 0];
                    span { (label) }
                }
            }
        }
    }
}

pub fn grid_preview(content: &Puzzle, as_partial: bool) -> Markup {
    let json = serde_json::to_string_pretty(content).unwrap_or_default();
    let xd = xd::write::write_xd(content);

    maybe_partial(as_partial, "#grid-preview", html! {
        div id="grid-preview" .preview {
            div.panel.grid-panel {
                (grid::grid_svg(content, Fill::Solution))
            }
            pre.panel.json-panel { code { (json) } }
            pre.panel.xd-panel { code { (xd) } }
        }
    })
}
