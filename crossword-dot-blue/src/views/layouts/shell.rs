use boutique::views::{Head, base};
use maud::{Markup, html};

use crate::{
    components::{footer, header},
    views::viewer::Viewer,
};

pub fn themed(head: Head, viewer: &Viewer) -> Head {
    match viewer.theme.attribute() {
        Some(name) => head.theme(name),
        None => head,
    }
}

pub fn shell(head: Head, viewer: &Viewer, content: Markup) -> Markup {
    base(
        &themed(head, viewer),
        html! {
            (header(viewer, true))
            (content)
            (footer())
        }
    )
}
