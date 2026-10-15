use bq_components::{Button, Input, Kind, html};
use maud::Markup;

use crate::views::{layouts::{head, shell}, viewer::Viewer};

pub fn page(viewer: &Viewer) -> Markup {
    shell(
        head("Get started").htmx(),
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
                        Input("email", kind = Kind::Email) placeholder="you@example.com" required;
                        Input("username", prefix = "@") placeholder="username" autocomplete="username" required;
                        Input("password", kind = Kind::Password, label = "Password") autocomplete="new-password" required;
                        Button(submit = true) .primary { "Create account" }

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
