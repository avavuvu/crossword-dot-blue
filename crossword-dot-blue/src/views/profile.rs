use maud::{Markup, html};

use crate::{
    models::{listing::{Entry, Filter}, puzzle::Region, user},
    views::{layouts::{page, shell}, listing, markdown, state::ViewState},
};

pub fn avatar(user: &user::Model) -> Markup {
    html! {
        @match user.avatar_url() {
            Some(url) => img.avatar src=(url) alt=(format!("{}'s avatar", user.display_name())) width="128" height="128";,
            None => div.avatar.placeholder aria-hidden="true" {
                (user.username.chars().next().map(|c| c.to_ascii_uppercase()).unwrap_or('?'))
            },
        }
    }
}

pub fn show(
    state: &ViewState,
    profile: &user::Model,
    filter: &Filter,
    regions: &[Region],
    entries: &[Entry],
) -> Markup {
    let path = profile.path();
    let is_owner = state.user_id.as_deref() == Some(profile.id.as_str());

    shell(
        page(format!("@{} — Crossword Dot Blue", profile.username)),
        state,
        html! {
            main.profile.listing {
                header.profile-header {
                    (avatar(profile))
                    div.identity {
                        h1 { (profile.display_name()) }
                        p.username { "@" (profile.username) }
                        @if let Some(bio) = profile.bio.as_deref() {
                            div.bio { (markdown::block(bio)) }
                        }
                        @if is_owner {
                            p { a.link href="/settings" { "Edit profile" } }
                        }
                    }
                }
                (listing::category_tabs_query(&path, filter))
                (listing::filter_form(&path, filter, regions))
                (listing::results(entries, state))
            }
        }
    )
}
