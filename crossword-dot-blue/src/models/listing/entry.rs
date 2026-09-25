use crate::models::{puzzle, user};

pub struct Entry {
    pub model: puzzle::Model,
    pub author: user::Model,
}
