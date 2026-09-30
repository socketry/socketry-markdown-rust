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

impl Handle for Text {
    fn handle(
        &self,
        state: &mut State,
        info: &Info,
        _parent: Option<&Node>,
        _node: &Node,
    ) -> Result<alloc::string::String, Message> {
        Ok(state.safe(&self.value, &SafeConfig::new(info.before, info.after, None)))
    }
}
