use bq_components::{Attrs, Button, component, html};
use maud::Markup;

#[component(eager)]
pub fn autosave_form(attrs: Attrs, children: Markup) -> Markup {
    html! {
        form
            data-autosave
            bq-setup
            bq-on:input="input"
            bq-on:htmx:before:request="sent"
            bq-on:htmx:response:error="failed"
            bq-on:beforeunload|window="unload"
            (..attrs)
        {
            (children)
        }
    }
}

#[component(eager)]
pub fn leave_link(#[positional] href: &str, attrs: Attrs, children: Markup) -> Markup {
    html! {
        Button(href = href) bq-setup bq-on:click="leave" (..attrs) {
            (children)
        }
    }
}
