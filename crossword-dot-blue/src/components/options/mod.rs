use bq_components::{component, html};
use maud::Markup;

use crate::theme::{THEMES, Theme};

#[component(setup = "theme-select")]
pub fn options_panel(#[positional] theme: Theme) -> Markup {
    html! {
        cw-options {
            form.options method="post" action="/options" bq-setup {
                div.field {
                    label for="option-theme" { "Theme" }
                    select id="option-theme" name="theme" bq-on:change="change" {
                        @for (name, label) in THEMES {
                            option value=(name) selected[*name == theme.name()] { (label) }
                        }
                    }
                }
                noscript {
                    button.button.primary.small type="submit" { "Save" }
                }
            }
        }
    }
}
