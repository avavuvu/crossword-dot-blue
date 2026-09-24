use boutique::components::Button;
use maud::{Markup, html};

use crate::{components::ui::copy_button, models::{puzzle, user}};

pub fn share_block(puzzle: &puzzle::Model, author: &user::Model) -> Markup {
    let share_url = format!("{}/share", puzzle.edit_path());
    let link = puzzle.link(&author.username);

    html! {
        section.share id="share" {
            h2 { "Share" }
            @if puzzle.is_public {
                p { "This puzzle is public. Anyone can play it at this link." }
            } @else if link.is_some() {
                p { "Anyone with this link can play the puzzle while it stays private." }
            } @else {
                p { "This puzzle is private and has no share link." }
            }

            @if let Some(url) = &link {
                div.share-link {
                    input type="text" readonly value=(url) aria-label="Link" data-select-all;
                    (copy_button(url, "Copy link"))
                }
            }

            @if !puzzle.is_public {
                div.share-actions {
                    @if link.is_some() {
                        (Button::button(html! { "Reset link" }).ghost().small()
                            .hx_post(&share_url)
                            .hx_target("#share")
                            .hx_swap("outerHTML")
                            .hx_confirm("Old links stop working. Continue?"))
                        (Button::button(html! { "Remove link" }).ghost().small()
                            .hx_delete(&share_url)
                            .hx_target("#share")
                            .hx_swap("outerHTML"))
                    } @else {
                        (Button::button(html! { "Create link" }).secondary().small()
                            .hx_post(&share_url)
                            .hx_target("#share")
                            .hx_swap("outerHTML"))
                    }
                }
            }
        }
    }
}
