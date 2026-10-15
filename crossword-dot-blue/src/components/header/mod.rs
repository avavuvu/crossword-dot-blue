use bq_components::{Button, component, html};
use maud::Markup;

use crate::{components::logo::{Layout, logo}, views::viewer::Viewer};

#[component(setup = "hero-watch")]
pub fn header(#[positional] viewer: &Viewer, #[positional] options_link: bool) -> Markup {
    html! {
        header.top bq-setup {
            a.logo href="/" {
                (logo(Layout::Small))
            }
            nav {
                @if viewer.is_authenticated() {
                    Button(href = "/app") .ghost .small { "Dashboard" }
                    Button(href = "/account") .ghost .small { "Account" }
                    Button(post = "/logout") .ghost .small { "Log out" }
                } @else {
                    Button(href = "/login") .ghost .small { "Log in" }
                    Button(href = "/signup") .primary .small { "Sign up" }
                }
                @if options_link {
                    a.button.ghost.small.options-link href="/options" aria-label="Options" title="Options" { "O" }
                }
            }
        }
    }
}
