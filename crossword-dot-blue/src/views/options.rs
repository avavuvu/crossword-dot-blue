use bq_components::html;
use maud::Markup;

use crate::{components::OptionsPanel, views::{layouts::{head, shell}, viewer::Viewer}};

pub fn page(viewer: &Viewer) -> Markup {
    shell(
        head("Options — Crossword Dot Blue"),
        viewer,
        html! {
            main.options-page {
                h1 { "Options" }
                OptionsPanel(viewer.theme);
            }
        }
    )
}
