use boutique::components::Button;
use maud::{Markup, html};

use crate::views::viewer::Viewer;

pub fn header(viewer: &Viewer) -> Markup {
    html! {
        header.top {
            a.logo href="/" {
                img src="/assets/images/small.svg" alt="Crossword.blue";
            }
            nav {
                @if viewer.is_authenticated() {
                    (Button::link(html! { "Dashboard" }, "/app").ghost().small())
                    (Button::link(html! { "Settings" }, "/settings").ghost().small())
                    (Button::post(html! { "Log out" }, "/logout").ghost().small())
                } @else {
                    (Button::link(html! { "Log in" }, "/login").ghost().small())
                    (Button::link(html! { "Sign up" }, "/signup").primary().small())
                }
            }
        }
    }
}
