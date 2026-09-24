use boutique::views::{Head, base};
use maud::{Markup, html};

use crate::{
    components::ui::{footer, header},
    views::state::ViewState,
};

pub fn shell(view: Head, state: &ViewState, content: Markup) -> Markup {
    base(
        &view,
        html! {
            (header::header(state))
            (content)
            (footer())
        }
    )
}
