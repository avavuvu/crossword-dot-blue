use maud::{Markup, html};

use crate::{components::options_panel, views::{layouts::{head, shell}, viewer::Viewer}};

pub fn page(viewer: &Viewer) -> Markup {
    shell(
        head("Options — Crossword Dot Blue"),
        viewer,
        html! {
            main.options-page {
                h1 { "Options" }
                (options_panel(viewer.theme))
            }
        }
    )
}
