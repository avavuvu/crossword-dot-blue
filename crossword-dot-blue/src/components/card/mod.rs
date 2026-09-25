use boutique::components::Button;
use crossword_macros::component;
use maud::{Markup, html};

use crate::{
    components::{copy_button, grid::{self, Fill}},
    models::{puzzle, user},
    views::viewer::Viewer,
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Actions {
    #[default]
    Browse,
    Manage,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Layout {
    #[default]
    Column,
    Row,
    RowReverse,
}

impl Layout {
    fn as_str(self) -> &'static str {
        match self {
            Layout::Column => "column",
            Layout::Row => "row",
            Layout::RowReverse => "row-reverse",
        }
    }
}

#[component]
pub fn card(
    #[builder(start_fn)] model: &puzzle::Model,
    #[builder(start_fn)] author: &user::Model,
    viewer: Option<&Viewer>,
    #[builder(default)] actions: Actions,
    #[builder(default)] fill: Fill,
    #[builder(default)] layout: Layout,
    #[builder(default)] tinted: bool,
) -> Markup {
    let key = model.key();
    let path = model.path(&author.username);
    let is_author = viewer
        .and_then(|viewer| viewer.user_id.as_deref())
        .is_some_and(|id| id == model.author_id);

    html! {
        cw-card
            id={ "model-" (key) }
            data-model-key=(key)
            layout=(layout.as_str())
            tinted[tinted]
        {
            (thumbnail(model, &path, fill))
            (body(model, author, &path, is_author, actions))
        }
    }
}

fn thumbnail(model: &puzzle::Model, path: &str, fill: Fill) -> Markup {
    html! {
        a.thumbnail href=(path) aria-hidden="true" tabindex="-1" {
            @if let Ok(puzzle) = model.puzzle() {
                @match fill {
                    Fill::Empty => (grid::thumbnail_svg(&puzzle)),
                    Fill::Solution => (grid::grid_svg(&puzzle, Fill::Solution)),
                }
            }
        }
    }
}

fn body(model: &puzzle::Model, author: &user::Model, path: &str, is_author: bool, actions: Actions) -> Markup {
    let (width, height) = model.dimensions();

    html! {
        div
            .body
            .featured[(model.is_featured())]
            .(format!("category-{}", model.category()))
            {
            h3 { a href=(path) { (model.display_title()) } }
            p.progress data-progress hidden {}
            @if actions == Actions::Browse {
                p.byline {
                    "by "
                    a href=(author.path()) { "@" (author.username) }
                }
            }
            p.details {
                span.category { (model.category().label()) }
                " · " (width) "×" (height)
                @if let Some(region) = &model.region { " · " (region) }
                @if let Some(difficulty) = model.difficulty { " · difficulty " (difficulty) }
            }
            @if actions == Actions::Manage {
                p.date { "Edited " (model.updated_at.format("%-d %b %Y")) }
            }
            @if model.is_cryptic || model.is_featured() {
                p.badges {
                    @if model.is_cryptic { span.badge.cryptic { "Cryptic" } }
                    @if model.is_featured() { span.badge.featured { "Featured " (model.category().label()) } }
                }
            }
            @match actions {
                Actions::Browse => @if is_author {
                    p.actions {
                        (Button::link(html! { "Edit" }, model.edit_path()).ghost().small())
                    }
                },
                Actions::Manage => (manage_actions(model, author, path)),
            }
        }
    }
}

fn manage_actions(model: &puzzle::Model, author: &user::Model, path: &str) -> Markup {
    let key = model.key();
    let edit_path = model.edit_path();

    html! {
        p.actions {
            (Button::link(html! { "Play" }, path).small())
            (Button::link(html! { "Edit" }, &edit_path).small())
            @if let Some(link) = model.link(&author.username) {
                (copy_button(&link, "Copy link"))
            }
            (Button::button(html! { "Delete" })
                .danger()
                .small()
                .hx_post(format!("{edit_path}/delete"))
                .hx_target(format!("#model-{key}"))
                .hx_swap("delete")
                .hx_confirm("Delete this model? Players lose their progress."))
        }
    }
}
