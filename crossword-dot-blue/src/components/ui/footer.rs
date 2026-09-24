use maud::{Markup, html};

pub fn footer() -> Markup {
    html! {
        footer.bottom {
            a.logo href="/" {
                picture {
                    source media="(width < 768px)" srcset="/assets/images/stacked.svg";
                    img src="/assets/images/small.svg" alt="Crossword.blue";
                }
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
