use boutique::components::{Button, Input};
use maud::{Markup, html};

use crate::views::{layouts::{HeadExt, head, shell}, viewer::Viewer};

pub fn page() -> Markup {
    shell(
        head("Log in").htmx().css("auth"),
        &Viewer::guest(),
        html! {
            main.auth {
                div.auth-form {
                    h1 { "Log in" }
                    p.error id="login-error" {}
                    form.flow
                        method="POST"
                        action="/login"
                        hx-post="/login"
                        hx-target="#login-error"
                    {
                        (Input::email("email").placeholder("you@example.com").required())
                        (Input::password("password").label("Password").required())
                        (Button::submit(html! { "Log in" }).primary())
                    }
                    p {
                        a.link href="/forgot-password" { "Forgot your password?" }
                    }
                    p {
                        "No account? "
                        a.link href="/signup" { "Sign up" }
                    }
                }
            }
        }
    )
}
