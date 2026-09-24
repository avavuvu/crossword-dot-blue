use boutique::components::Button;
use maud::{Markup, html};

use crate::models::{listing::Filter, puzzle::Region};

pub fn filter_form(action: &str, filter: &Filter, regions: &[Region]) -> Markup {
    let current = filter.region();

    html! {
        form.filters method="GET" action=(action) {
            @if let Some(category) = filter.category() {
                input type="hidden" name="category" value=(category.as_str());
            }
            label {
                span { "Region" }
                select name="region" {
                    option value="" selected[current.is_none()] { "Any region" }
                    @for region in regions {
                        option value=(region.as_str()) selected[current.as_ref() == Some(region)] { (region) }
                    }
                }
            }
            (Button::submit(html! { "Filter" }).secondary().small())
            @if current.is_some() || filter.cryptic() {
                (Button::link(html! { "Clear" }, action).ghost().small())
            }
        }
    }
}
