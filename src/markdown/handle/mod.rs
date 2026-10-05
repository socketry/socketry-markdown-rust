// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2026, by Samuel Williams.

use crate::markdown::{state::Info, State};
use crate::{mdast::Node, message::Message};
use alloc::string::String;

mod blockquote;
mod r#break;
mod code;
mod definition;
pub mod emphasis;
mod frontmatter;
mod heading;
pub mod html;
pub mod image;
pub mod image_reference;
pub mod inline_code;
pub mod inline_math;
pub mod link;
pub mod link_reference;
mod list;
mod list_item;
mod math;
mod paragraph;
mod root;
pub mod strong;
mod text;
mod thematic_break;

pub trait Handle {
    fn handle(
        &self,
        state: &mut State,
        info: &Info,
        parent: Option<&Node>,
        node: &Node,
    ) -> Result<String, Message>;
}
