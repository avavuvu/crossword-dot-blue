use bq_components::{attrs, button};
use maud::{Markup, Render, html};

pub fn copy_button(url: &str, label: &str) -> Markup {
    button(html! { (label) }).small().attrs(attrs!(data_copy = url)).render()
}

pub fn share_button(url: &str) -> Markup {
    button(html! { "Share" })
        .ghost()
        .small()
        .attrs(attrs!(data_copy = url, data_share, data_copied = "Link copied"))
        .render()
}
