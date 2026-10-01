// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/handle/thematic-break.js
use super::Handle;
use crate::markdown::{
    state::{Info, State},
    util::{check_rule::check_rule, check_rule_repetition::check_rule_repetition},
};
use crate::{
    mdast::{Node, ThematicBreak},
    message::Message,
};
use alloc::format;

impl Handle for ThematicBreak {
    fn handle(
        &self,
        state: &mut State,
        _info: &Info,
        _parent: Option<&Node>,
        _node: &Node,
    ) -> Result<alloc::string::String, Message> {
        let marker = check_rule(state)?;
        let space = if state.options.rule_spaces { " " } else { "" };
        let mut value =
            format!("{}{}", marker, space).repeat(check_rule_repetition(state)? as usize);

        if state.options.rule_spaces {
            // Remove the last space.
            value.pop();
            Ok(value)
        } else {
            Ok(value)
        }
    }
}
