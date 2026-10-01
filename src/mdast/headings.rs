// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Helpers for extracting headings from an mdast tree.
use super::Node;
use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    vec::Vec,
};

/// Options for extracting headings from a document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeadingOptions {
    /// The shallowest heading level to include (between `1` and `6`).
    pub min_level: u8,
    /// The deepest heading level to include (between `1` and `6`).
    pub max_level: u8,
}

impl Default for HeadingOptions {
    fn default() -> Self {
        Self {
            min_level: 1,
            max_level: 6,
        }
    }
}

/// A heading extracted from an mdast tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadingEntry<'a> {
    /// The original heading node.
    pub node: &'a Node,
    /// The heading level, between `1` and `6`.
    pub level: u8,
    /// The heading's text content.
    pub text: String,
    /// A unique, lowercase anchor derived from the heading text.
    pub anchor: String,
}

/// Headings extracted from a Markdown document.
///
/// This helper is useful for building a table of contents. It walks the tree
/// in document order, converts each heading to plain text, and assigns unique
/// anchors to repeated headings. Anchor assignment includes headings filtered
/// out by [`HeadingOptions`], so the remaining anchors still match rendered
/// headings when [`CompileOptions::heading_ids`][crate::CompileOptions::heading_ids]
/// is enabled.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Headings<'a> {
    entries: Vec<HeadingEntry<'a>>,
}

impl<'a> Headings<'a> {
    /// Extract all headings from a document tree.
    #[must_use]
    pub fn extract(root: &'a Node) -> Self {
        Self::extract_with_options(root, &HeadingOptions::default())
    }

    /// Extract headings from a document tree with a level range.
    #[must_use]
    pub fn extract_with_options(root: &'a Node, options: &HeadingOptions) -> Self {
        let mut result = Self::default();
        let mut anchors = AnchorGenerator::default();

        root.walk(|node| {
            if let Node::Heading(heading) = node {
                let text = chomp_line_ending(&node.text_content()).to_string();
                let anchor = anchors.anchor_for(&text);

                if heading.depth >= options.min_level && heading.depth <= options.max_level {
                    result.entries.push(HeadingEntry {
                        node,
                        level: heading.depth,
                        text,
                        anchor,
                    });
                }
            }
        });

        result
    }

    /// Return the extracted entries as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[HeadingEntry<'a>] {
        &self.entries
    }

    /// Iterate over the extracted entries in document order.
    pub fn iter(&self) -> core::slice::Iter<'_, HeadingEntry<'a>> {
        self.entries.iter()
    }

    /// Return the number of extracted headings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the document contains no headings in the selected level range.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<'a, 'b> IntoIterator for &'b Headings<'a> {
    type Item = &'b HeadingEntry<'a>;
    type IntoIter = core::slice::Iter<'b, HeadingEntry<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl<'a> IntoIterator for Headings<'a> {
    type Item = HeadingEntry<'a>;
    type IntoIter = alloc::vec::IntoIter<HeadingEntry<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[derive(Default)]
struct AnchorGenerator {
    next_suffix: BTreeMap<String, usize>,
    used: BTreeSet<String>,
}

impl AnchorGenerator {
    fn anchor_for(&mut self, text: &str) -> String {
        let base = slug(text);
        let mut suffix = self.next_suffix.get(&base).copied().unwrap_or(2);
        let mut anchor = base.clone();

        while self.used.contains(&anchor) {
            anchor = alloc::format!("{base}-{suffix}");
            suffix += 1;
        }

        self.next_suffix.insert(base, suffix);
        self.used.insert(anchor.clone());
        anchor
    }
}

fn slug(text: &str) -> String {
    let mut result = String::new();
    let mut previous_was_whitespace = false;
    let lowercase = text.to_lowercase();

    for character in lowercase.chars() {
        if character.is_whitespace() {
            if !previous_was_whitespace {
                result.push('-');
            }
            previous_was_whitespace = true;
        } else {
            result.push(character);
            previous_was_whitespace = false;
        }
    }

    result
}

fn chomp_line_ending(text: &str) -> &str {
    if let Some(text) = text.strip_suffix("\r\n") {
        text
    } else if let Some(text) = text.strip_suffix('\n') {
        text
    } else if let Some(text) = text.strip_suffix('\r') {
        text
    } else {
        text
    }
}
