use boutique::views::base;
use bq_components::html;
use crossword_tools::puzzle::Puzzle;
use maud::Markup;

use crate::{
    components::{CopyButton, Crossword, Header, footer},
    models::{puzzle, user},
    views::{layouts::{head, themed}, viewer::Viewer},
};

pub fn page(model: &puzzle::Model, author: &user::Model, puzzle: &Puzzle, viewer: &Viewer) -> Markup {
    let title = model.display_title();

    base(
        &themed(head(format!("{title} — Crossword Dot Blue")).css("play"), viewer),
        html! {
            article.crossword {
                div.above {
                    Header(viewer, false);
                    hgroup {
                        h1 { (title) }
                        p.byline {
                            "by "
                            a href=(author.path()) { "@" (author.username) }
                        }
                        @if model.is_public {
                            CopyButton(&model.url(&author.username), share = true, copied = "Link copied") .ghost { "Share" }
                        }
                    }
                }
                Crossword(model, puzzle, options = viewer.theme);
            }
            (footer())
        }
    )
}
