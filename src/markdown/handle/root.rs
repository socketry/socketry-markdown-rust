// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/handle/root.js
use super::Handle;
use crate::markdown::state::{is_phrasing, Info, State};
use crate::{
    mdast::{Node, Root},
    message::Message,
};
use alloc::string::String;

impl Handle for Root {
    fn handle(
        &self,
        state: &mut State,
        info: &Info,
        _parent: Option<&Node>,
        node: &Node,
    ) -> Result<String, Message> {
        let has_phrasing = self.children.iter().any(is_phrasing);

        if has_phrasing {
            state.container_phrasing(node, info)
        } else {
            state.container_flow(node)
        }
    }
}
