use serde::Deserialize;

use super::Entry;
use crate::models::{category::Category, region::Region};

#[derive(Deserialize, Default, Clone)]
pub struct Filter {
    pub category: Option<String>,
    pub region: Option<String>,
    #[serde(default)]
    pub cryptic: Option<String>,
}

impl Filter {
    pub fn with_category(mut self, category: Category) -> Self {
        self.category = Some(category.as_str().to_string());
        self
    }

    pub fn category(&self) -> Option<Category> {
        self.category.as_deref().and_then(Category::parse)
    }

    pub fn region(&self) -> Option<Region> {
        self.region.as_deref().and_then(Region::parse)
    }

    pub fn cryptic(&self) -> bool {
        self.cryptic.as_deref().is_some_and(|value| !value.is_empty() && value != "0")
    }

    pub fn matches(&self, entry: &Entry) -> bool {
        let category = self.category().is_none_or(|category| entry.puzzle.category() == category);
        let region = self.region().is_none_or(|region| entry.puzzle.region.as_ref() == Some(&region));
        let cryptic = !self.cryptic() || entry.puzzle.is_cryptic;
        category && region && cryptic
    }

    pub fn apply(&self, entries: Vec<Entry>) -> Vec<Entry> {
        entries.into_iter().filter(|entry| self.matches(entry)).collect()
    }

    pub fn query_string(&self) -> String {
        self.query_string_with(None)
    }

    pub fn query_string_with(&self, category: Option<Category>) -> String {
        let mut parts = Vec::new();
        if let Some(category) = category {
            parts.push(format!("category={}", category.as_str()));
        }
        if let Some(region) = self.region() {
            parts.push(format!("region={}", region.as_str()));
        }
        if self.cryptic() {
            parts.push("cryptic=1".to_string());
        }
        if parts.is_empty() { String::new() } else { format!("?{}", parts.join("&")) }
    }
}
