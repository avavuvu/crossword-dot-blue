use boutique::views::{Head, base};
use maud::{Markup, html};

use crate::{
    components::{footer, header},
    views::viewer::Viewer,
};

pub fn dashboard_shell(head: Head, viewer: &Viewer, content: Markup) -> Markup {
    base(
        &head,
        html! {
            (header(viewer))
            (content)
            (footer())
        }
    )
}
