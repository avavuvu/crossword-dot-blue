use boutique::components::{Button, Input};
use maud::{Markup, html};

use crate::views::{layouts::{head, shell}, viewer::Viewer};

pub fn page() -> Markup {
    shell(
        head("Get started").htmx(),
        &Viewer::guest(),
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
                        (Input::email("email").placeholder("you@example.com").required())
                        (Input::text("username").prefix("@").placeholder("username").autocomplete("username").required())
                        (Input::password("password").label("Password").autocomplete("new-password").required())
                        (Button::submit(html! { "Create account" }).primary())

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
