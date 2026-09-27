use boutique::views::base;
use crossword_tools::puzzle::Puzzle;
use maud::{Markup, html};

use crate::{components::crossword, models::{puzzle}, views::{layouts::head}};

pub fn page(model: &puzzle::Model, puzzle: &Puzzle) -> Markup {
    base(
        &head("Crossword dot blue"),
        html! {
            (
                crossword(model, puzzle)
                    .tagline(html! {
                        "Powered by "
                        a href="https://crossword.blue" {
                            "Crossword dot blue"
                        }
                    })
            )
        }
    )
}
