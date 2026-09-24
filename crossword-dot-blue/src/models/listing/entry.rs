use crate::models::{puzzle, user};

pub struct Entry {
    pub puzzle: puzzle::Model,
    pub author: user::Model,
}

impl Entry {
    pub fn path(&self) -> String {
        self.puzzle.path(&self.author.username)
    }
}
