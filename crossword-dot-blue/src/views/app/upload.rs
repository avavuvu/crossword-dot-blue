use boutique::components::Button;
use maud::{Markup, html};

pub fn form() -> Markup {
    let accept = crossword_tools::EXTENSIONS
        .iter()
        .map(|ext| format!(".{ext}"))
        .collect::<Vec<_>>()
        .join(",");

    html! {
        form.upload
            method="POST"
            action="/app/upload"
            enctype="multipart/form-data"
            hx-post="/app/upload"
            hx-trigger="change, submit"
            hx-swap="none"
        {
            label.dropzone for="file" {
                input type="file" id="file" name="file" accept=(accept) required;
                span.prompt { "Choose a file or drop it here" }
                span.formats { "Accepts .ipuz, .puz and .xd" }
            }

            p.error id="file-error" {}

            noscript {
                (Button::submit(html! { "Upload" }))
            }
        }
    }
}
