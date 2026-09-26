use boutique::{components::Button, htmx::partial};
use maud::{Markup, html};

use crate::{
    cloudinary,
    components::markdown,
    models::user,
    views::{layouts::{HeadExt, head, shell}, profile::avatar, viewer::Viewer},
};

pub fn save_status(user: &user::Model) -> Markup {
    let saved = user.updated_at.unwrap_or(user.created_at);
    html! {
        "Saved at "
        time datetime=(saved.to_rfc3339()) { (saved.format("%-d %b %Y, %H:%M")) }
    }
}

pub fn avatar_block(user: &user::Model) -> Markup {
    partial("#avatar", html! {
        section.avatar-section id="avatar" {
            h2 { "Profile picture" }
            div.avatar-row {
                (avatar(user))
                @if cloudinary::is_configured() {
                    form.avatar-form
                        method="POST"
                        action="/account/avatar"
                        enctype="multipart/form-data"
                        hx-post="/account/avatar"
                        hx-encoding="multipart/form-data"
                        hx-trigger="change, submit"
                        hx-target="#avatar"
                        hx-swap="outerHTML"
                    {
                        label.dropzone for="avatar-file" {
                            input type="file" id="avatar-file" name="avatar" accept="image/jpeg,image/png,image/webp" required;
                            span.prompt { "Choose an image" }
                            span.formats { "JPEG, PNG or WebP, up to 5 MB" }
                        }
                        p.error id="avatar-error" {}
                        noscript { (Button::submit(html! { "Upload" })) }
                    }
                } @else {
                    p.hint { "Image uploads are not available right now." }
                }
            }
        }
    })
}

pub fn page(user: &user::Model) -> Markup {
    shell(
        head("Account — Crossword Dot Blue").htmx().entry("editor").css("account"),
        &Viewer::from(user),
        html! {
            main.account {
                header.account-header {
                    h1 { "Account settings" }
                    p.subtitle { a href=(user.path()) { "@" (user.username) } }
                }

                form.profile-form
                    method="POST"
                    action="/account"
                    hx-post="/account"
                    hx-trigger="change delay:500ms, submit"
                    hx-sync="this:replace"
                    hx-target="#account-status"
                    data-autosave
                {
                    div.input-component {
                        label for="display_name" { "Display name" }
                        input id="display_name" name="display_name" type="text" maxlength="60"
                            value=(user.display_name.as_deref().unwrap_or(""))
                            placeholder=(user.username)
                            autocomplete="nickname";
                        p.error id="display_name-error" {}
                    }

                    div.input-component {
                        label for="bio" { "Bio" }
                        (markdown(html! {
                            textarea id="bio" name="bio" rows="5" maxlength="1000" {
                                (user.bio.as_deref().unwrap_or(""))
                            }
                        }).hint(true).bordered(true))
                        p.error id="bio-error" {}
                    }

                    p.save-status id="account-status" {
                        @if user.updated_at.is_some() { (save_status(user)) }
                    }

                    noscript { (Button::submit(html! { "Save" }).primary()) }
                }

                (avatar_block(user))
            }
        }
    )
}
