use boutique::{components::Button, htmx::partial};
use maud::{Markup, html};

use crate::{
    cloudinary,
    components::ui::markdown_textarea,
    models::user,
    views::{layouts::{page, shell}, profile::avatar, state::ViewState},
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
                        action="/settings/avatar"
                        enctype="multipart/form-data"
                        hx-post="/settings/avatar"
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

pub fn show(user: &user::Model) -> Markup {
    shell(
        page("Settings — Crossword Dot Blue").htmx().module("/assets/editor.js"),
        &ViewState::from(user),
        html! {
            main.settings {
                header.settings-header {
                    h1 { "Settings" }
                    p.subtitle { a href=(user.path()) { "@" (user.username) } }
                }

                form.profile-form
                    method="POST"
                    action="/settings"
                    hx-post="/settings"
                    hx-trigger="change delay:500ms, submit"
                    hx-sync="this:replace"
                    hx-target="#settings-status"
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
                        (markdown_textarea(true, html! {
                            textarea id="bio" name="bio" rows="5" maxlength="1000" data-markdown {
                                (user.bio.as_deref().unwrap_or(""))
                            }
                        }))
                        p.error id="bio-error" {}
                    }

                    p.save-status id="settings-status" {
                        @if user.updated_at.is_some() { (save_status(user)) }
                    }

                    noscript { (Button::submit(html! { "Save" }).primary()) }
                }

                (avatar_block(user))
            }
        }
    )
}
