use bq_components::setup;
use maud::{Markup, html};

use crate::theme::{THEMES, Theme};

setup!(ThemeSelect);

pub fn options_panel(theme: Theme) -> Markup {
    html! {
        cw-options {
            form.options method="post" action="/options" bq-setup=(ThemeSelect) {
                div.field {
                    label for="option-theme" { "Theme" }
                    select id="option-theme" name="theme" bq-ref="select" {
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
