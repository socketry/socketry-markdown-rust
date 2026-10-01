// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/handle/emphasis.js
use super::Handle;
use crate::markdown::{
    construct_name::ConstructName,
    state::{Info, State},
    util::check_emphasis::check_emphasis,
};
use crate::{
    mdast::{Emphasis, Node},
    message::Message,
};
use alloc::format;

impl Handle for Emphasis {
    fn handle(
        &self,
        state: &mut State,
        info: &Info,
        _parent: Option<&Node>,
        node: &Node,
    ) -> Result<alloc::string::String, Message> {
        let marker = check_emphasis(state)?;

        state.enter(ConstructName::Emphasis);

        let mut value = format!("{}{}", marker, state.container_phrasing(node, info)?);
        value.push(marker);

        state.exit();

        Ok(value)
    }
}

pub fn peek_emphasis(state: &State) -> char {
    state.options.emphasis
}
