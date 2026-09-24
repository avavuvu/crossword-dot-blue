use boutique::views::Head;

pub fn page(title: impl Into<String>) -> Head {
    Head::new(title)
        .stylesheet("/assets/app.css")
        .module("/assets/site.js")
}
