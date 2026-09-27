use boutique::views::Head;

pub fn head(title: impl Into<String>) -> Head {
    Head::new(title).favicon("/assets/favicon.ico").entry("site")
}
