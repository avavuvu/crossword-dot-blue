use maud::{Markup, html};

use crate::components::logo::{Layout, logo};

pub fn footer() -> Markup {
    html! {
        footer.bottom {
            div.logo {
                (logo(Layout::Small))
                (logo(Layout::Stacked))
            }

            div.info {
                div {
                    p {
                        "Made with love by Ava Dinh-Vu"
                    }
                    p {
                        a.link href="/about" { "About" }
                        // " · "
                        // a.link href="/privacy" { "Privacy" }
                    }
                }
            }
        }
    }
}
