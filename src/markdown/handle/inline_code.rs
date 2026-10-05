// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/handle/inline-code.js
use super::Handle;
use crate::markdown::state::{Info, State};
use crate::{
    mdast::{InlineCode, Node},
    message::Message,
};
use alloc::{format, string::String};
use regex::Regex;

impl Handle for InlineCode {
    fn handle(
        &self,
        state: &mut State,
        _info: &Info,
        _parent: Option<&Node>,
        _node: &Node,
    ) -> Result<alloc::string::String, Message> {
        let mut value = self.value.clone();
        let mut sequence = String::from('`');
        let mut grave_accent_match = Regex::new(&format!(r"(^|[^`]){}([^`]|$)", sequence)).unwrap();
        while grave_accent_match.is_match(&value) {
            sequence.push('`');
            grave_accent_match = Regex::new(&format!(r"(^|[^`]){}([^`]|$)", sequence)).unwrap();
        }

        let no_whitespaces = !value.chars().all(char::is_whitespace);
        let starts_with_whitespace = value.starts_with(char::is_whitespace);
        let ends_with_whitespace = value.ends_with(char::is_whitespace);
        let starts_with_tick = value.starts_with('`');
        let ends_with_tick = value.ends_with('`');

        if no_whitespaces
            && ((starts_with_whitespace && ends_with_whitespace)
                || starts_with_tick
                || ends_with_tick)
        {
            value = format!("{}{}{}", ' ', value, ' ');
        }

        for pattern in &mut state.r#unsafe {
            if !pattern.at_break {
                continue;
            }

            State::compile_pattern(pattern);

            let regex = pattern
                .compiled
                .as_ref()
                .expect("compiling an unsafe pattern sets its regex");
            while let Some(m) = regex.find(&value) {
                let match_start = m.start();
                let (start, end) = if match_start > 0
                    && value.as_bytes()[match_start - 1] == b'\r'
                    && value.as_bytes()[match_start] == b'\n'
                {
                    (match_start - 1, match_start + 1)
                } else {
                    (match_start, match_start + 1)
                };

                value.replace_range(start..end, " ");
            }
        }

        let prefix = self
            .lang
            .as_ref()
            .map(|lang| format!("{}:", lang))
            .unwrap_or_default();

        Ok(format!("{}{}{}{}", prefix, sequence, value, sequence))
    }
}

pub fn peek_inline_code() -> char {
    '`'
}

#[cfg(test)]
mod tests {
    use super::Handle;
    use crate::{
        markdown::{
            r#unsafe::Unsafe,
            state::{Info, State},
            Options,
        },
        mdast::{InlineCode, Node},
    };
    use alloc::{string::String, vec};
    use regex::Regex;

    #[test]
    fn replaces_a_crlf_line_ending_with_one_space() {
        let options = Options::default();
        let mut state = State::new(&options);
        let mut line_ending = Unsafe::new('\n', None, None, vec![], vec![], true);
        line_ending.set_compiled(Regex::new("\\n").unwrap());
        state.r#unsafe = vec![line_ending];
        let inline_code = InlineCode {
            value: String::from("one\r\ntwo"),
            position: None,
            lang: None,
        };
        let node = Node::InlineCode(inline_code.clone());

        assert_eq!(
            inline_code
                .handle(&mut state, &Info::new("", ""), None, &node)
                .unwrap(),
            "`one two`"
        );
    }

    #[test]
    fn preserves_inline_code_when_no_unsafe_break_pattern_matches() {
        let options = Options::default();
        let mut state = State::new(&options);
        state.r#unsafe = vec![Unsafe::new('#', None, None, vec![], vec![], true)];
        let inline_code = InlineCode {
            value: String::from("ordinary code"),
            position: None,
            lang: None,
        };
        let node = Node::InlineCode(inline_code.clone());

        assert_eq!(
            inline_code
                .handle(&mut state, &Info::new("", ""), None, &node)
                .unwrap(),
            "`ordinary code`"
        );
    }
}
