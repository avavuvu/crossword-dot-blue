use boutique::views::Head;

use crate::assets;

pub fn page(title: impl Into<String>) -> Head {
    Head::new(title)
        .stylesheet(assets::url("app"))
        .module(assets::url("site"))
}
