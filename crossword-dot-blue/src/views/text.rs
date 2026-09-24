use maud::{Markup, PreEscaped, html};

use crate::{
    handlers::text::TextPage,
    views::{layouts::{page, shell}, state::ViewState},
};

pub fn show(text: &TextPage, state: &ViewState) -> Markup {
    shell(
        page(format!("{} — Crossword Dot Blue", text.title)),
        state,
        html! {
            main.text {
                article.prose { (PreEscaped(text.html)) }
            }
        }
    )
}
