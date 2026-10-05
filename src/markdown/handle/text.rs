// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/handle/text.js
use super::Handle;
use crate::markdown::{
    state::{Info, State},
    util::safe::SafeConfig,
};
use crate::{
    mdast::{Node, Text},
    message::Message,
};
use alloc::{borrow::Cow, string::String};

impl Handle for Text {
    fn handle(
        &self,
        state: &mut State,
        info: &Info,
        _parent: Option<&Node>,
        _node: &Node,
    ) -> Result<alloc::string::String, Message> {
        let value = match state.options.line_wrapping {
            crate::markdown::LineWrapping::Preserve => Cow::Borrowed(&self.value),
            crate::markdown::LineWrapping::Unwrap => {
                Cow::Owned(unwrap_soft_line_breaks(&self.value))
            }
        };

        Ok(state.safe(&value, &SafeConfig::new(info.before, info.after, None)))
    }
}

fn unwrap_soft_line_breaks(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut characters = value.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            '\r' | '\n' => {
                if character == '\r' && characters.peek() == Some(&'\n') {
                    characters.next();
                }

                while matches!(result.chars().next_back(), Some(' ' | '\t')) {
                    result.pop();
                }

                while characters
                    .peek()
                    .is_some_and(|next| matches!(*next, ' ' | '\t'))
                {
                    characters.next();
                }

                result.push(' ');
            }
            _ => result.push(character),
        }
    }

    result
}
