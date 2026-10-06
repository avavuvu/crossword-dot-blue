use bq_components::button;
use maud::{Markup, html};

use crate::models::{listing::Filter, region::Region};

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
            (button(html! { "Filter" }).submit().secondary().small())
            @if current.is_some() || filter.cryptic() {
                (button(html! { "Clear" }).href(action).ghost().small())
            }
        }
    }
}
