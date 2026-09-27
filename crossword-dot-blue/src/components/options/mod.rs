use maud::{Markup, html};

use crate::theme::{THEMES, Theme};

pub fn options_panel(theme: Theme) -> Markup {
    html! {
        cw-options {
            form.options method="post" action="/options" {
                div.field {
                    label for="option-theme" { "Theme" }
                    select id="option-theme" name="theme" {
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
