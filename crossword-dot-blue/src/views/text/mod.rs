use maud::{Markup, PreEscaped, html};

use crate::views::{layouts::{head, shell}, viewer::Viewer};

pub struct TextPage {
    pub title: &'static str,
    pub html: &'static str,
}

pub fn page(text: &TextPage, viewer: &Viewer) -> Markup {
    shell(
        head(format!("{} — Crossword Dot Blue", text.title)),
        viewer,
        html! {
            main.text {
                article.prose { (PreEscaped(text.html)) }
            }
        }
    )
}
