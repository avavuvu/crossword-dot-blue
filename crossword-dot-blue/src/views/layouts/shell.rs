use boutique::views::{Head, base};
use bq_components::html;
use maud::Markup;

use crate::{
    components::{Header, footer},
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
            Header(viewer, true);
            (content)
            (footer())
        }
    )
}
