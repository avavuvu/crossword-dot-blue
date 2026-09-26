use maud::{Markup, html};

use crate::{
    components::{Actions, Layout, card},
    models::{puzzle, user},
    components::grid::Fill,
    views::{self, layouts::{HeadExt, dashboard_shell, head}, viewer::Viewer},
};

pub fn page(user: &user::Model, puzzles: Vec<puzzle::Model>) -> Markup {
    let (public, private): (Vec<_>, Vec<_>) = puzzles.into_iter().partition(|p| p.is_public);
    let viewer = Viewer::from(user);

    dashboard_shell(
        head("Dashboard — Crossword Dot Blue").htmx().css("dashboard"),
        &viewer,
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
                    (puzzle_list(user, &viewer, &private))
                }

                section.puzzle-section.public {
                    header.section-header {
                        h2 { "Public" }
                    }
                    (puzzle_list(user, &viewer, &public))
                }
            }
        }
    )
}

fn puzzle_list(user: &user::Model, viewer: &Viewer, puzzles: &[puzzle::Model]) -> Markup {
    html! {
        @if puzzles.is_empty() {
            div.empty-viewer {
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
                    li {
                        (card(puzzle, user)
                            .viewer(viewer)
                            .actions(Actions::Manage)
                            .fill(Fill::Solution)
                            .layout(Layout::Row))
                    }
                }
            }
        }
    }
}
