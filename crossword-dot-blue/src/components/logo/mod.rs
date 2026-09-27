use maud::{Markup, PreEscaped};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    Small,
    Stacked,
}

impl Layout {
    const fn source(self) -> &'static str {
        match self {
            Layout::Small => include_str!("small.svg"),
            Layout::Stacked => include_str!("stacked.svg"),
        }
    }
}

pub fn logo(layout: Layout) -> Markup {
    PreEscaped(layout.source().trim_end().to_string())
}
