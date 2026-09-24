use boutique::components::Button;
use maud::{Markup, html};

use crate::{
    components::ui::copy_button,
    models::{puzzle, user},
    views::{self, layouts::{dashboard_shell, page}, state::ViewState},
};

pub fn index(user: &user::Model, puzzles: Vec<puzzle::Model>) -> Markup {
    let (public, private): (Vec<_>, Vec<_>) = puzzles.into_iter().partition(|p| p.is_public);

    dashboard_shell(
        page("Dashboard — Crossword Dot Blue").htmx(),
        &ViewState::from(user),
        html! {
            main.dashboard {
                div {
                    header.dashboard-header {
                        div {
                            h1 { "Your puzzles" }
                            p.subtitle { "@" (user.username) }
                        }
                    }

                    section.upload {
                        details.upload-panel id="upload" {
                            summary { span.button { "Upload a puzzle" } }
                            (views::app::upload::form())
                        }
                    }
                }

                section.puzzle-section.private {
                    header.section-header {
                        h2 { "Private" }
                    }
                    (puzzle_list(user, &private))
                }

                section.puzzle-section.public {
                    header.section-header {
                        h2 { "Public" }
                    }
                    (puzzle_list(user, &public))
                }
            }
        }
    )
}

fn puzzle_list(user: &user::Model, puzzles: &[puzzle::Model]) -> Markup {
    html! {
        @if puzzles.is_empty() {
            div.empty-state {
                p {
                    "No puzzles yet. "
                    a href="#upload" data-open="upload" {
                        "Upload a puzzle to get started"
                    }
                }
            }
        } @else {
            ul.puzzle-list {
                @for puzzle in puzzles {
                    (puzzle_row(user, puzzle))
                }
            }
        }
    }
}

pub fn puzzle_row(user: &user::Model, puzzle: &puzzle::Model) -> Markup {
    let key = puzzle.key();
    let view_url = puzzle.path(&user.username);
    let edit_url = puzzle.edit_path();

    let primary_url = if puzzle.is_public { &view_url } else { &edit_url };

    let (width, height) = puzzle.dimensions();
    let content = &puzzle.content();

    html! {
        li.puzzle-row id={ "puzzle-" (key) } {
            @if let Ok(puzzle) = content {
                a.grid-container href=(primary_url) {
                    (views::grid::grid_svg(puzzle, views::grid::Fill::Solution))
                }
            }

            div.puzzle-meta {
                h3 { a href=(edit_url) { (puzzle.display_title()) } }
                p.details {
                    (width) "×" (height)
                    @if let Some(difficulty) = puzzle.difficulty { " · difficulty " (difficulty) }
                    @if let Some(region) = &puzzle.region { " · " (region) }
                }
                p.date { "Edited " (puzzle.updated_at.format("%-d %b %Y")) }
            }

            div.puzzle-actions {
                (Button::link(html! { "Play" }, &view_url).small())
                (Button::link(html! { "Edit" }, &edit_url).small())
                @if let Some(link) = puzzle.link(&user.username) {
                    (copy_button(&link, "Copy link"))
                }
                (Button::button(html! { "Delete" })
                    .danger()
                    .small()
                    .hx_post(format!("{edit_url}/delete"))
                    .hx_target(format!("#puzzle-{key}"))
                    .hx_swap("delete")
                    .hx_confirm("Delete this puzzle? Players lose their progress."))
            }
        }
    }
}
