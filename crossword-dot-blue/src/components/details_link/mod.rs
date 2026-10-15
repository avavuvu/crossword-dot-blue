use bq_components::{Attrs, component, html};
use maud::Markup;

#[component]
pub fn details_link(#[positional] target: &str, attrs: Attrs, children: Markup) -> Markup {
    html! {
        a href={ "#" (target) } data-open=(target) bq-setup bq-on:click="open" (..attrs) {
            (children)
        }
    }
}
