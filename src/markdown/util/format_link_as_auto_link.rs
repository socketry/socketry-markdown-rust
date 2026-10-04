// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/util/format-link-as-autolink.js
use crate::markdown::state::State;
use crate::mdast::{Link, Node, Text};
use alloc::{format, string::ToString};
use regex::RegexBuilder;

pub fn auto_link_text<'a>(link: &Link, node: &'a Node, state: &State) -> Option<&'a Text> {
    let raw = node.to_string();

    let children = node.children()?;
    if children.len() != 1 {
        return None;
    }

    let Node::Text(text) = &children[0] else {
        return None;
    };

    let mailto = format!("mailto:{}", raw);
    let start_with_protocol = RegexBuilder::new("^[a-z][a-z+.-]+:")
        .case_insensitive(true)
        .build()
        .unwrap();

    (!state.options.resource_link
        && !link.url.is_empty()
        && link.title.is_none()
        && (raw == link.url || mailto == link.url)
        && start_with_protocol.is_match(&link.url)
        && is_valid_url(&link.url))
    .then_some(text)
}

fn is_valid_url(url: &str) -> bool {
    !url.chars()
        .any(|c| c.is_control() || c.is_whitespace() || c == '<' || c == '>')
}

#[cfg(test)]
mod tests {
    use super::auto_link_text;
    use crate::{
        markdown::{state::State, Options},
        mdast::{Link, Node, Text},
    };
    use alloc::vec;

    #[test]
    fn selects_only_safe_single_text_autolinks() {
        let link = Link {
            children: vec![Node::Text(Text {
                value: "https://example.com".into(),
                position: None,
            })],
            position: None,
            url: "https://example.com".into(),
            title: None,
        };
        let node = Node::Link(link.clone());
        let default_options = Options::default();
        let state = State::new(&default_options);

        assert!(auto_link_text(&link, &node, &state).is_some());

        let link = Link {
            url: "mailto:person@example.com".into(),
            children: vec![Node::Text(Text {
                value: "person@example.com".into(),
                position: None,
            })],
            ..link.clone()
        };
        assert!(auto_link_text(&link, &Node::Link(link.clone()), &state).is_some());

        assert!(auto_link_text(
            &link,
            &Node::Text(Text {
                value: "plain".into(),
                position: None,
            }),
            &state
        )
        .is_none());

        let options = Options {
            resource_link: true,
            ..Options::default()
        };
        assert!(auto_link_text(&link, &Node::Link(link.clone()), &State::new(&options)).is_none());
    }
}
