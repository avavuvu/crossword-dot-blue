use regex::Regex;

use super::types::{Direction, Puzzle};
use crate::utils::generate_clue_id;


impl Puzzle {
    pub fn link_clues(&mut self) {
        let reference = Regex::new(r"(?i)\b(\d+)[ -]?(across|down)\b").unwrap();
        let mut pairs = Vec::new();

        for (from, clue) in &self.clues {
            for capture in reference.captures_iter(&clue.body) {
                let Ok(number) = capture[1].parse::<u16>() else {
                    continue;
                };
                let direction = if capture[2].eq_ignore_ascii_case("across") {
                    Direction::Across
                } else {
                    Direction::Down
                };
                let to = generate_clue_id(direction, number);

                if to != *from && self.clues.contains_key(&to) {
                    pairs.push((from.clone(), to));
                }
            }
        }

        for (from, to) in pairs {
            if let Some(clue) = self.clues.get_mut(&from) {
                push_unique(&mut clue.refs, to.clone());
            }
            if let Some(clue) = self.clues.get_mut(&to) {
                push_unique(&mut clue.refs, from);
            }
        }
    }
}

fn push_unique(refs: &mut Vec<String>, id: String) {
    if !refs.contains(&id) {
        refs.push(id);
    }
}
