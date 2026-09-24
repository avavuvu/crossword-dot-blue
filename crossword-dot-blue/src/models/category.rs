use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Mini,
    Midi,
    Big,
}

impl Category {
    pub const ALL: [Category; 3] = [Category::Mini, Category::Midi, Category::Big];

    pub fn as_str(self) -> &'static str {
        match self {
            Category::Mini => "mini",
            Category::Midi => "midi",
            Category::Big => "big",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Category::Mini => "Mini",
            Category::Midi => "Midi",
            Category::Big => "Big",
        }
    }

    pub fn parse(value: &str) -> Option<Category> {
        Category::ALL
            .into_iter()
            .find(|category| category.as_str().eq_ignore_ascii_case(value.trim()))
    }

    pub fn browse_path(self) -> String {
        format!("/browse/{}", self.as_str())
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
