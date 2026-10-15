use boutique::views::{Head, base};
use bq_components::html;
use maud::Markup;

use crate::{
    components::{Header, footer},
    views::{layouts::themed, viewer::Viewer},
};

pub fn dashboard_shell(head: Head, viewer: &Viewer, content: Markup) -> Markup {
    base(
        &themed(head, viewer),
        html! {
            Header(viewer, true);
            (content)
            (footer())
        }
    )
}
