use markdown::{Constructs, ParseOptions, mdast::Node, to_mdast};
use maud::{Markup, html};

fn options() -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            attention: true,
            label_start_link: true,
            label_end: true,
            character_escape: true,
            character_reference: true,

            autolink: false,
            block_quote: false,
            code_indented: false,
            code_fenced: false,
            code_text: false,
            definition: false,
            hard_break_escape: false,
            hard_break_trailing: false,
            heading_atx: false,
            heading_setext: false,
            html_flow: false,
            html_text: false,
            label_start_image: false,
            list_item: false,
            thematic_break: false,
            ..Constructs::default()
        },
        ..ParseOptions::default()
    }
}

pub fn inline(text: &str) -> Markup {
    match parse(text) {
        Some(root) => render(&root),
        None => html! { (text) },
    }
}

pub fn block(text: &str) -> Markup {
    match parse(text) {
        Some(Node::Root(root)) => html! {
            @for child in &root.children {
                @match child {
                    Node::Paragraph(p) => p { (children(&p.children)) },
                    other => p { (render(other)) },
                }
            }
        },
        _ => html! { p { (text) } },
    }
}

fn parse(text: &str) -> Option<Node> {
    let escaped = text.replace('_', "\\_");
    to_mdast(&escaped, &options()).ok()
}

fn render(node: &Node) -> Markup {
    match node {
        Node::Root(root) => html! {
            @for (index, child) in root.children.iter().enumerate() {
                @if index > 0 { " " }
                (render(child))
            }
        },
        Node::Paragraph(p) => children(&p.children),
        Node::Text(t) => html! { (t.value) },
        Node::Strong(s) => html! { strong { (children(&s.children)) } },
        Node::Emphasis(e) => html! { em { (children(&e.children)) } },
        Node::Link(link) if safe_href(&link.url) => html! {
            a href=(link.url) { (children(&link.children)) }
        },
        Node::Link(link) => children(&link.children),
        other => html! { (other.to_string()) },
    }
}

fn children(nodes: &[Node]) -> Markup {
    html! {
        @for child in nodes {
            (render(child))
        }
    }
}

fn safe_href(href: &str) -> bool {
    let lower = href.trim().to_ascii_lowercase();
    lower.starts_with("https://")
        || lower.starts_with("http://")
        || lower.starts_with("mailto:")
        || (lower.starts_with('/') && !lower.starts_with("//"))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    const FIXTURES: &str = include_str!("../../tests/markdown.json");

    #[derive(Deserialize)]
    struct Case {
        input: String,
        output: String,
    }

    #[test]
    fn inline_matches_fixtures() {
        let cases: Vec<Case> = serde_json::from_str(FIXTURES).expect("fixtures parse");
        for case in cases {
            assert_eq!(super::inline(&case.input).into_string(), case.output, "input: {:?}", case.input);
        }
    }
}
