use maud::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    ArrowLeftRight,
    Delete,
    Grid,
    CrosswordMini,
    CrosswordMidi,
    CrosswordBig,
    CrosswordSolved,
}

impl Icon {
    pub const fn source(self) -> &'static str {
        match self {
            Icon::ArrowLeftRight => include_str!("arrow-left-right.svg"),
            Icon::Delete => include_str!("delete.svg"),
            Icon::Grid => include_str!("grid.svg"),
            Icon::CrosswordMini => include_str!("crossword-mini.svg"),
            Icon::CrosswordMidi => include_str!("crossword-midi.svg"),
            Icon::CrosswordBig => include_str!("crossword-big.svg"),
            Icon::CrosswordSolved => include_str!("crossword-solved.svg"),
        }
    }
}

impl Render for Icon {
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str(self.source().trim_end());
    }
}
