use bq_components::{button, input, Kind};
use maud::{Markup, html};

use crate::views::{layouts::{head, shell}, viewer::Viewer};

pub fn page(viewer: &Viewer) -> Markup {
    shell(
        head("Get started").htmx().css("auth"),
        viewer,
        html! {
            main.auth {
                div.auth-form {
                    h1 { "Create account" }
                    p.error id="signup-error" {}
                    form.flow
                        method="POST"
                        action="/signup"
                        hx-post="/signup"
                        hx-target="#signup-error"
                    {
                        (input("email").kind(Kind::Email).placeholder("you@example.com").required())
                        (input("username").prefix("@").placeholder("username").autocomplete("username").required())
                        (input("password").kind(Kind::Password).label("Password").autocomplete("new-password").required())
                        (button(html! { "Create account" }).submit().primary())

                        p {
                            "Already have an account? "
                            a.link href="/login" { "Sign in" }
                        }
                    }
                }
            }
        }
    )
}
