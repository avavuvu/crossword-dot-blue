use bq_components::{button, input, Kind};
use maud::{Markup, html};

use crate::views::{layouts::{head, shell}, viewer::Viewer};

pub fn page(viewer: &Viewer) -> Markup {
    shell(
        head("Log in").htmx().css("auth"),
        viewer,
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
                        (input("email").kind(Kind::Email).placeholder("you@example.com").required())
                        (input("password").kind(Kind::Password).label("Password").required())
                        (button(html! { "Log in" }).submit().primary())
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
