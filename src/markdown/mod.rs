// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! API.
//!
//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/index.js.

// This serializer is ported from `mdast-util-to-markdown`; retain its established
// implementation while keeping the parser's stricter lints on the surrounding API.
#![allow(clippy::pedantic)]

pub use self::configure::{IndentOptions, LineWrapping, Options};
use self::state::{Info, State};
use crate::{mdast::Node, message::Message};
use alloc::string::String;

extern crate alloc;
mod association;
mod configure;
mod construct_name;
mod handle;
mod state;
mod r#unsafe;
mod util;

/// Turn an mdast syntax tree into markdown.
///
/// # Errors
///
/// Returns an error if the AST contains a node that cannot be serialized.
pub fn to_markdown(tree: &Node) -> Result<String, Message> {
    to_markdown_with_options(tree, &Options::default())
}

/// Turn an mdast syntax tree, with options, into markdown.
///
/// # Errors
///
/// Returns an error if an option contains an invalid marker or the AST contains
/// a node that cannot be serialized.
pub fn to_markdown_with_options(tree: &Node, options: &Options) -> Result<String, Message> {
    let mut state = State::new(options);
    let mut result = state.handle(tree, &Info::new("\n", "\n"), None)?;

    if !result.is_empty() {
        let last_char = result.chars().last().unwrap();
        if last_char != '\n' && last_char != '\r' {
            result.push('\n');
        }
    }

    Ok(result)
}
