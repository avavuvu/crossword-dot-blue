use bq_components::{button, setup};
use maud::{Markup, html};

use crate::{components::logo::{Layout, logo}, views::viewer::Viewer};

setup!(HeroWatch);

pub fn header(viewer: &Viewer, options_link: bool) -> Markup {
    html! {
        header.top bq-setup=(HeroWatch) {
            a.logo href="/" {
                (logo(Layout::Small))
            }
            nav {
                @if viewer.is_authenticated() {
                    (button(html! { "Dashboard" }).href("/app").ghost().small())
                    (button(html! { "Account" }).href("/account").ghost().small())
                    (button(html! { "Log out" }).post("/logout").ghost().small())
                } @else {
                    (button(html! { "Log in" }).href("/login").ghost().small())
                    (button(html! { "Sign up" }).href("/signup").primary().small())
                }
                @if options_link {
                    a.button.ghost.small.options-link href="/options" aria-label="Options" title="Options" { "O" }
                }
            }
        }
    }
}
