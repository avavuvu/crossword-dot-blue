use boutique::views::Head;

use crate::assets;

pub trait HeadExt {
    fn entry(self, name: &str) -> Self;
    fn css(self, name: &str) -> Self;
}

impl HeadExt for Head {
    fn entry(mut self, name: &str) -> Self {
        for href in assets::styles(name) {
            self = self.stylesheet(href);
        }
        match assets::url(name) {
            Some(src) => self.module(src),
            None => self,
        }
    }

    fn css(self, name: &str) -> Self {
        match assets::url(name) {
            Some(href) => self.stylesheet(href),
            None => self,
        }
    }
}

pub fn head(title: impl Into<String>) -> Head {
    Head::new(title).favicon("/assets/favicon.ico").entry("site")
}
