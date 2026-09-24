use boutique::components::{Button, Input};
use maud::{Markup, html};

use crate::views::{layouts::{page, shell}, state::ViewState};

pub fn login() -> Markup {
    shell(
        page("Log in").htmx(),
        &ViewState::guest(),
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

pub fn signup() -> Markup {
    shell(
        page("Get started").htmx(),
        &ViewState::guest(),
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
