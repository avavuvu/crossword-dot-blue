use boutique::{ErrorReport, views::error_content};
use maud::{Markup, html};

use crate::views::{layouts::{head, shell}, viewer::Viewer};

pub fn page(report: &ErrorReport, back: &str, viewer: &Viewer) -> Markup {
    shell(
        head(format!("{} — Crossword Dot Blue", report.status)),
        viewer,
        html! {
            main.text {
                article.prose { (error_content(report, back)) }
            }
        }
    )
}
