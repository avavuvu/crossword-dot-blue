use crossword_tools::puzzle::{Cell, Direction, Puzzle, Style};
use maud::{Markup, html};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Fill {
    Empty,
    Solution,
}

fn cell_path(puzzle: &Puzzle, include: impl Fn(&Cell) -> bool) -> String {
    let width = puzzle.width;
    let mut path = String::new();
    for (index, cell) in puzzle.cells.iter().enumerate() {
        if include(cell) {
            path.push_str(&format!("M{} {}h1v1h-1z", index % width, index / width));
        }
    }
    path
}

fn grid_lines(width: usize, height: usize) -> String {
    let mut path = String::new();
    for x in 1..width {
        path.push_str(&format!("M{x} 0v{height}"));
    }
    for y in 1..height {
        path.push_str(&format!("M0 {y}h{width}"));
    }
    path
}

pub fn thumbnail_svg(puzzle: &Puzzle) -> Markup {
    let blocks = cell_path(puzzle, Cell::is_block);
    let shaded = cell_path(puzzle, |cell| !cell.is_block() && cell.styles().contains(&Style::Shaded));
    let lines = grid_lines(puzzle.width, puzzle.height);

    html! {
        svg.grid.thumbnail
            xmlns="http://www.w3.org/2000/svg"
            viewBox={ "0 0 " (puzzle.width) " " (puzzle.height) }
            preserveAspectRatio="xMidYMid slice"
            role="img"
            aria-label="Crossword grid"
        {
            @if !shaded.is_empty() {
                path.shaded d=(shaded) {}
            }
            @if !blocks.is_empty() {
                path.blocks d=(blocks) {}
            }
            path.lines d=(lines) {}
        }
    }
}

pub fn grid_svg(puzzle: &Puzzle, fill: Fill) -> Markup {
    let filled = fill == Fill::Solution;
    let width = puzzle.width;
    let height = puzzle.height;
    let position = |index: usize| (index % width, index / width);

    html! {
        svg.grid
            xmlns="http://www.w3.org/2000/svg"
            viewBox={ "0 0 " (width) " " (height) }
            role="img"
            aria-label="Crossword grid"
        {
            g.cells {
                @for (index, cell) in puzzle.cells.iter().enumerate() {
                    @let (x, y) = position(index);
                    @let styles = cell.styles();
                    rect.cell
                        .block[cell.is_block()]
                        .shaded[styles.contains(&Style::Shaded)]
                        .empty[styles.contains(&Style::Empty)]
                        data-index=(index)
                        x=(x) y=(y) width="1" height="1" {}
                }
            }

            g.circles {
                @for (index, cell) in puzzle.cells.iter().enumerate() {
                    @if cell.styles().contains(&Style::Circle) {
                        @let (x, y) = position(index);
                        circle.circle cx=(x as f32 + 0.5) cy=(y as f32 + 0.5) r="0.45" {}
                    }
                }
            }

            g.numbers {
                @for (index, cell) in puzzle.cells.iter().enumerate() {
                    @if let Cell::Letter { number: Some(number), .. } = cell {
                        @let (x, y) = position(index);
                        text.number x=(x as f32 + 0.06) y=(y as f32 + 0.3) { (number) }
                    }
                }
            }

            g.prefilled {
                @for (index, cell) in puzzle.cells.iter().enumerate() {
                    @if let Cell::Letter { prefilled: Some(text), .. } = cell {
                        @let (x, y) = position(index);
                        text.prefilled x=(x as f32 + 0.5) y=(y as f32 + 0.8) { (text) }
                    }
                }
            }

            g.entries {
                @for (index, cell) in puzzle.cells.iter().enumerate() {
                    @if let Cell::Letter { prefilled: None, solution, .. } = cell {
                        @let (x, y) = position(index);
                        text.entry.rebus[filled && solution.chars().count() > 1] data-entry=(index) x=(x as f32 + 0.5) y=(y as f32 + 0.8) {
                            @if filled { (solution) }
                        }
                    }
                }
            }

            g.splits {
                @for clue in puzzle.clues.values() {
                    @for &split in &clue.splits {
                        @if let Some(&index) = clue.indexes.get(split.wrapping_sub(1)) {
                            @let (x, y) = position(index);
                            @match clue.direction {
                                Direction::Across => {
                                    line.split x1=(x + 1) y1=(y) x2=(x + 1) y2=(y + 1) {}
                                }
                                Direction::Down => {
                                    line.split x1=(x) y1=(y + 1) x2=(x + 1) y2=(y + 1) {}
                                }
                            }
                        }
                    }
                }
            }

            rect.outline x="0" y="0" width=(width) height=(height) {}
        }
    }
}
