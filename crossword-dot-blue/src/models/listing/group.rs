use super::Entry;
use crate::models::{category::Category, region::Region};

pub fn in_category(entries: &[Entry], category: Category) -> Vec<&Entry> {
    entries.iter().filter(|entry| entry.puzzle.category() == category).collect()
}

pub fn regions(entries: &[Entry]) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    for entry in entries {
        if let Some(region) = &entry.puzzle.region {
            if !regions.contains(region) {
                regions.push(region.clone());
            }
        }
    }
    regions.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    regions
}
