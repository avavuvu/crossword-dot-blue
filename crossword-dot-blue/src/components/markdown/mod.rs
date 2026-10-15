use bq_components::{component, html};
use maud::Markup;

#[component(setup = "markdown-highlight")]
pub fn markdown(
    #[positional] textarea: Markup,
    hint: bool,
    bordered: bool,
) -> Markup {
    html! {
        cw-markdown bordered[bordered] bq-setup {
            div.editor {
                pre.backdrop aria-hidden="true" hx-morph-skip bq-ref="backdrop" {}
                (textarea)
            }
            @if hint {
                p.hint {
                    "Supports "
                    span.color { "**" }
                    strong { "bold" }
                    span.color { "**" }
                    ", "
                    span.color { "*" }
                    em { "italic" }
                    span.color { "*" }
                    ", and "
                    span.a { "[" }
                    "links"
                    span.a { "](https://…)" }
                }
            }
        }
    }
}
