// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/handle/strong.js
use super::Handle;
use crate::markdown::{
    construct_name::ConstructName,
    state::{Info, State},
    util::check_strong::check_strong,
};
use crate::{
    mdast::{Node, Strong},
    message::Message,
};
use alloc::format;

impl Handle for Strong {
    fn handle(
        &self,
        state: &mut State,
        info: &Info,
        _parent: Option<&Node>,
        node: &Node,
    ) -> Result<alloc::string::String, Message> {
        let marker = check_strong(state)?;

        state.enter(ConstructName::Strong);

        let mut value = format!(
            "{}{}{}",
            marker,
            marker,
            state.container_phrasing(node, info)?
        );
        value.push(marker);
        value.push(marker);

        state.exit();

        Ok(value)
    }
}

pub fn peek_strong(state: &State) -> char {
    state.options.strong
}
