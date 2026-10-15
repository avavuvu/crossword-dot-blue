use bq_components::{Attrs, Button, component, html};
use maud::Markup;

#[component]
pub fn copy_button(
    #[positional] url: &str,
    share: bool,
    copied: Option<&str>,
    attrs: Attrs,
    children: Markup,
) -> Markup {
    html! {
        Button .small data-copy=(url) data-share[share] data-copied=[copied] bq-setup bq-on:click="copy" (..attrs) {
            (children)
        }
    }
}

#[component]
pub fn copy_link(#[positional] url: &str, attrs: Attrs) -> Markup {
    html! {
        div bq-setup (..attrs) {
            input type="text" readonly value=(url) aria-label="Link" bq-on:click="select";
            CopyButton(url) { "Copy link" }
        }
    }
}
