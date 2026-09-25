use crossword_macros::component;
use maud::{Markup, html};

#[component]
pub fn markdown(
    #[builder(start_fn)] textarea: Markup,
    #[builder(default)] hint: bool,
    #[builder(default)] bordered: bool,
) -> Markup {
    html! {
        cw-markdown bordered[bordered] {
            div.editor {
                pre.backdrop aria-hidden="true" hx-morph-skip {}
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
