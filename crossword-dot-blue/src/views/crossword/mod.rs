use boutique::views::base;
use crossword_tools::puzzle::Puzzle;
use maud::{Markup, html};

use crate::{
    components::{crossword, footer, header, share_button},
    models::{puzzle, user},
    views::{layouts::{HeadExt, head}, viewer::Viewer},
};

pub fn page(model: &puzzle::Model, author: &user::Model, puzzle: &Puzzle, viewer: &Viewer) -> Markup {
    let title = model.display_title();

    base(
        &head(format!("{title} — Crossword Dot Blue")).css("play"),
        html! {
            article.crossword {
                div.above {
                    (header(viewer))
                    hgroup {
                        h1 { (title) }
                        p.byline {
                            "by "
                            a href=(author.path()) { "@" (author.username) }
                        }
                        @if model.is_public {
                            (share_button(&model.url(&author.username)))
                        }
                    }
                }
                (crossword(model, puzzle))
            }
            (footer())
        }
    )
}
