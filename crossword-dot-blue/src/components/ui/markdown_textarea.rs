use maud::{Markup, html};

pub fn markdown_textarea(include_hint: bool, textarea: Markup) -> Markup {
    html! {
        div.highlighted-textarea {
            pre.backdrop aria-hidden="true" hx-morph-skip {}
            (textarea)
        }
        @if include_hint {
            p.hint {
                "Supports "
                span.color { "**" }
                b { "bold" }
                span.color { "**"}
                ", "
                span.color { "*" }
                i { "italic" }
                span.color {"*"}
                ", and "
                span.a { "[" }
                "links"
                span.a { "](https://…)" }
            }
        }
    }
}
