use maud::{Markup, html};

use crate::models::{listing::Filter, puzzle::Category};

fn tabs(current: Option<Category>, href: impl Fn(Option<Category>) -> String) -> Markup {
    html! {
        nav.category-tabs aria-label="Size" {
            a.tab.active[current.is_none()] href=(href(None)) { "All" }
            @for category in Category::ALL {
                a.tab.active[current == Some(category)] href=(href(Some(category))) { (category.label()) }
            }
        }
    }
}

pub fn category_tabs(base: &str, current: Option<Category>, filter: &Filter) -> Markup {
    let query = filter.query_string();
    let base = base.trim_end_matches('/');

    tabs(current, |category| match category {
        Some(category) => format!("{base}/{}{query}", category.as_str()),
        None => format!("{base}{query}"),
    })
}

pub fn category_tabs_query(base: &str, filter: &Filter) -> Markup {
    tabs(filter.category(), |category| format!("{base}{}", filter.query_string_with(category)))
}
