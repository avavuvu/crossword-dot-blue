use bq_components::Button;
use maud::{Markup, html};

use crate::{components::logo::{Layout, logo}, views::viewer::Viewer};

pub fn header(viewer: &Viewer, options_link: bool) -> Markup {
    html! {
        header.top {
            a.logo href="/" {
                (logo(Layout::Small))
            }
            nav {
                @if viewer.is_authenticated() {
                    (Button::link(html! { "Dashboard" }, "/app").ghost().small())
                    (Button::link(html! { "Account" }, "/account").ghost().small())
                    (Button::post(html! { "Log out" }, "/logout").ghost().small())
                } @else {
                    (Button::link(html! { "Log in" }, "/login").ghost().small())
                    (Button::link(html! { "Sign up" }, "/signup").primary().small())
                }
                @if options_link {
                    a.button.ghost.small.options-link href="/options" aria-label="Options" title="Options" { "O" }
                }
            }
        }
    }
}
