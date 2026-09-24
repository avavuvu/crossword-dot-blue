use boutique::components::Button;
use maud::{Markup, Render, html};

use crate::{
    models::listing::Entry,
    views::{grid, state::ViewState},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Column,
    Row,
    RowReverse,
}

pub struct Card<'a> {
    entry: &'a Entry,
    layout: Layout,
    viewer: Option<&'a ViewState>,
    standalone: bool,
}

impl<'a> Card<'a> {
    pub fn new(entry: &'a Entry) -> Self {
        Self {
            entry,
            layout: Layout::Column,
            viewer: None,
            standalone: false,
        }
    }

    pub fn row(mut self) -> Self {
        self.layout = Layout::Row;
        self
    }

    pub fn row_reverse(mut self) -> Self {
        self.layout = Layout::RowReverse;
        self
    }


    pub fn viewer(mut self, state: &'a ViewState) -> Self {
        self.viewer = Some(state);
        self
    }


    pub fn standalone(mut self) -> Self {
        self.standalone = true;
        self
    }

    fn is_author(&self) -> bool {
        self.viewer
            .and_then(|state| state.user_id.as_deref())
            .is_some_and(|id| id == self.entry.puzzle.author_id)
    }

    fn thumbnail(&self, path: &str) -> Markup {
        html! {
            a.thumbnail href=(path) aria-hidden="true" tabindex="-1" {
                @if let Ok(content) = self.entry.puzzle.content() {
                    (grid::thumbnail_svg(&content))
                }
            }
        }
    }

    fn body(&self, path: &str) -> Markup {
        let puzzle = &self.entry.puzzle;
        let author = &self.entry.author;
        let (width, height) = puzzle.dimensions();

        html! {
            div
                .card-body
                .featured[(puzzle.is_featured())]
                .(format!("category-{}", puzzle.category()))
                {
                h3 { a href=(path) { (puzzle.display_title()) } }
                p.progress data-progress hidden {}
                p.byline {
                    "by "
                    a href=(author.path()) { "@" (author.username) }
                }
                p.details {
                    span.category { (puzzle.category().label()) }
                    " · " (width) "×" (height)
                    @if let Some(region) = &puzzle.region { " · " (region) }
                    @if let Some(difficulty) = puzzle.difficulty { " · difficulty " (difficulty) }
                }
                @if puzzle.is_cryptic || puzzle.is_featured() {
                    p.badges {
                        @if puzzle.is_cryptic { span.badge.cryptic { "Cryptic" } }
                        @if puzzle.is_featured() { span.badge.featured { "Featured " (puzzle.category().label()) } }
                    }
                }
                @if self.is_author() {
                    p.card-actions {
                        (Button::link(html! { "Edit" }, puzzle.edit_path()).ghost().small())
                    }
                }
            }
        }
    }
}

impl Render for Card<'_> {
    fn render(&self) -> Markup {
        let path = self.entry.path();
        let row = matches!(self.layout, Layout::Row | Layout::RowReverse);
        let reverse = self.layout == Layout::RowReverse;
        let inner = html! {
            (self.thumbnail(&path))
            (self.body(&path))
        };

        let key = self.entry.puzzle.key();

        html! {
            @if self.standalone {
                div.puzzle-card.row[row].reverse[reverse] data-puzzle-key=(key) { (inner) }
            } @else {
                li.puzzle-card.row[row].reverse[reverse] data-puzzle-key=(key) { (inner) }
            }
        }
    }
}
