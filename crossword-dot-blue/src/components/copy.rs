use boutique::components::Button;
use maud::{Markup, Render, html};

pub fn copy_button(url: &str, label: &str) -> Markup {
    Button::button(html! { (label) }).small().attr("data-copy", url).render()
}

pub fn share_button(url: &str) -> Markup {
    Button::button(html! { "Share" })
        .ghost()
        .small()
        .attr("data-copy", url)
        .attr("data-share", "")
        .attr("data-copied", "Link copied")
        .render()
}
