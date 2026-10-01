// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/util/check-bullet-ordered.js
use crate::markdown::state::State;
use crate::message::Message;
use alloc::{boxed::Box, format};

pub fn check_bullet_ordered(state: &mut State) -> Result<char, Message> {
    let marker = state.options.bullet_ordered;

    if marker != '.' && marker != ')' {
        return Err(Message {
            place: None,
            reason: format!(
                "Cannot serialize items with `{}` for `options.bullet_ordered`, expected `.` or `)`",
                marker
            ),
            rule_id: Box::new("unexpected-marker".into()),
            source: Box::new("mdast-util-to-markdown".into()),
        });
    }

    Ok(marker)
}
