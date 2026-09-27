use boutique::views::{Head, base};
use maud::{Markup, html};

use crate::{
    components::{footer, header},
    views::{layouts::themed, viewer::Viewer},
};

pub fn dashboard_shell(head: Head, viewer: &Viewer, content: Markup) -> Markup {
    base(
        &themed(head, viewer),
        html! {
            (header(viewer, true))
            (content)
            (footer())
        }
    )
}
