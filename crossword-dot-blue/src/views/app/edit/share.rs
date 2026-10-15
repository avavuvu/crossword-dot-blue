use bq_components::{Button, html};
use maud::Markup;

use crate::{components::CopyLink, models::{puzzle, user}};

pub fn share_block(model: &puzzle::Model, author: &user::Model) -> Markup {
    let share_url = format!("{}/share", model.edit_path());
    let link = model.link(&author.username);

    html! {
        section.share id="share" {
            h2 { "Share" }
            @if model.is_public {
                p { "This puzzle is public. Anyone can play it at this link." }
            } @else if link.is_some() {
                p { "Anyone with this link can play the puzzle while it stays private." }
            } @else {
                p { "This puzzle is private and has no share link." }
            }

            @if let Some(url) = &link {
                CopyLink(url) .share-link;
            }

            @if !model.is_public {
                div.share-actions {
                    @if link.is_some() {
                        Button .ghost .small
                            hx-post=(share_url)
                            hx-target="#share"
                            hx-swap="outerHTML"
                            hx-confirm="Old links stop working. Continue?"
                        {
                            "Reset link"
                        }
                        Button .ghost .small hx-delete=(share_url) hx-target="#share" hx-swap="outerHTML" { "Remove link" }
                    } @else {
                        Button .secondary .small hx-post=(share_url) hx-target="#share" hx-swap="outerHTML" { "Create link" }
                    }
                }
            }
        }
    }
}
