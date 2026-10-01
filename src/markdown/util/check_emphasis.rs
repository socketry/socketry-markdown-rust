// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/util/check-emphasis.js
use crate::markdown::state::State;
use crate::message::Message;
use alloc::{boxed::Box, format};

pub fn check_emphasis(state: &State) -> Result<char, Message> {
    let marker = state.options.emphasis;

    if marker != '*' && marker != '_' {
        return Err(Message {
            place: None,
            reason: format!(
                "Cannot serialize emphasis with `{}` for `options.emphasis`, expected `*`, or `_`",
                marker
            ),
            rule_id: Box::new("unexpected-marker".into()),
            source: Box::new("mdast-util-to-markdown".into()),
        });
    }

    Ok(marker)
}
