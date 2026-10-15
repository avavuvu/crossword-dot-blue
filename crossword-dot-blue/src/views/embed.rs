use boutique::views::base;
use bq_components::html;
use crossword_tools::puzzle::Puzzle;
use maud::Markup;

use crate::{components::Crossword, models::{puzzle}, views::{layouts::head}};

pub fn page(model: &puzzle::Model, puzzle: &Puzzle) -> Markup {
    base(
        &head("Crossword dot blue"),
        html! {
            Crossword(model, puzzle, tagline = html! {
                "Powered by "
                a href="https://crossword.blue" {
                    "Crossword dot blue"
                }
            });
        }
    )
}
