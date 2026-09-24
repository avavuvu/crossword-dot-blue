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
            Icon::ArrowLeftRight => include_str!("../../../resources/icons/arrow-left-right.svg"),
            Icon::Delete => include_str!("../../../resources/icons/delete.svg"),
            Icon::Grid => include_str!("../../../resources/icons/grid.svg"),
            Icon::CrosswordMini => include_str!("../../../resources/icons/crossword-mini.svg"),
            Icon::CrosswordMidi => include_str!("../../../resources/icons/crossword-midi.svg"),
            Icon::CrosswordBig => include_str!("../../../resources/icons/crossword-big.svg"),
            Icon::CrosswordSolved => include_str!("../../../resources/icons/crossword-solved.svg"),
        }
    }
}

impl Render for Icon {
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str(self.source().trim_end());
    }
}
