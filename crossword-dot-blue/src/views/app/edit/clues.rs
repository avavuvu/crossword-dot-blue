use boutique::htmx::partial;
use crossword_tools::puzzle::{Cell, Clue, Direction, Puzzle};
use maud::{Markup, html};

use crate::components::ui::markdown_textarea;

pub fn clue_list(key: &str, puzzle: &Puzzle) -> Markup {
    html! {
        section.clues {
            (clue_group(key, puzzle, Direction::Across, "Across"))
            (clue_group(key, puzzle, Direction::Down, "Down"))
        }
    }
}

fn clue_group(key: &str, puzzle: &Puzzle, direction: Direction, heading: &str) -> Markup {
    let clues = puzzle
        .clue_order
        .iter()
        .filter_map(|id| puzzle.clues.get(id))
        .filter(|clue| clue.direction == direction);

    html! {
        div.clue-group {
            h2 { (heading) }
            ol {
                @for clue in clues {
                    (clue_row(key, puzzle, clue, None, false))
                }
            }
        }
    }
}

pub fn clue_row(key: &str, puzzle: &Puzzle, clue: &Clue, error: Option<&str>, as_partial: bool) -> Markup {
    let action = clue_url(key, clue, "");
    let id = format!("clue-{}", clue.id);

    let row = html! {
        li.clue-row id=(id) {
            form
                hx-post=(action)
                hx-trigger="change delay:500ms, submit"
                hx-sync="this:replace"
                hx-target="closest li"
                hx-swap="outerMorph"
                data-autosave
            {
                div {
                    span.number {
                        (clue.number)"."
                    }

                    span.answer {
                        (answer(key, puzzle, clue))
                    }
                }

                div {
                    (markdown_textarea(false, html! {
                        textarea.clue-body
                            name="body"
                            rows="1"
                            data-markdown
                            aria-label="Clue"
                            placeholder="Clue"
                            data-submit-on-enter
                        { (clue.body) }
                    }))
                }

                (refs(key, puzzle, clue))

                @if let Some(error) = error {
                    p.error { (error) }
                }
            }
        }
    };

    if as_partial { partial(&format!("#{id}"), row) } else { row }
}

fn clue_url(key: &str, clue: &Clue, suffix: &str) -> String {
    format!("/app/edit/{key}/clues/{}{suffix}", clue.id)
}

fn answer(key: &str, puzzle: &Puzzle, clue: &Clue) -> Markup {
    html! {
        @for (position, &index) in clue.indexes.iter().enumerate() {
            @if position > 0 {
                @let split = clue.splits.contains(&position);
                button.gap.split[split]
                    type="button"
                    hx-post=(clue_url(key, clue, &format!("/split/{position}")))
                    hx-target="closest li"
                    hx-swap="outerMorph"
                    aria-label=(if split { "Remove split" } else { "Add split" })
                    aria-pressed=(split)
                {}
            }
            span.letter { (solution(puzzle, index)) }
        }

    }
}

fn refs(key: &str, puzzle: &Puzzle, clue: &Clue) -> Markup {
    html! {
        div.refs {
            @for ref_id in &clue.refs {
                span.ref {
                    (ref_label(ref_id))
                    button.remove
                        type="button"
                        hx-post=(clue_url(key, clue, &format!("/refs/{ref_id}/remove")))
                        hx-target="closest li"
                        hx-swap="outerMorph"
                        aria-label={ "Unlink " (ref_label(ref_id)) }
                    { "×" }
                }
            }
            select.add-ref name="add_ref" aria-label="Link to clue" {
                option value="" selected { "Link clue…" }
                @for id in &puzzle.clue_order {
                    @let taken = id == &clue.id || clue.refs.contains(id);
                    option value=(id) disabled[taken] { (ref_label(id)) }
                }
            }
        }
    }
}

pub fn ref_label(id: &str) -> String {
    match id.split_at_checked(1) {
        Some(("A", number)) => format!("{number} Across"),
        Some(("D", number)) => format!("{number} Down"),
        _ => id.to_string(),
    }
}

fn solution(puzzle: &Puzzle, index: usize) -> &str {
    match puzzle.cells.get(index) {
        Some(Cell::Letter { solution, .. }) => solution,
        _ => "",
    }
}
