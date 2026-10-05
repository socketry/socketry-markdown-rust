// Released under the MIT License.
// Copyright, 2024, by Bnchi.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

//! JS equivalent: https://github.com/syntax-tree/mdast-util-math/blob/main/lib/index.js#L241
use super::Handle;
use crate::markdown::state::{Info, State};
use crate::{
    mdast::{InlineMath, Node},
    message::Message,
};
use alloc::format;
use regex::Regex;

impl Handle for InlineMath {
    fn handle(
        &self,
        state: &mut State,
        _info: &Info,
        _parent: Option<&Node>,
        _node: &Node,
    ) -> Result<alloc::string::String, Message> {
        let mut size: usize = if !state.options.single_dollar_text_math {
            2
        } else {
            1
        };

        let pattern = format!("(^|[^$]){}([^$]|$)", "\\$".repeat(size));
        let mut dollar_sign_match = Regex::new(&pattern).unwrap();
        while dollar_sign_match.is_match(&self.value) {
            size += 1;
            let pattern = format!("(^|[^$]){}([^$]|$)", "\\$".repeat(size));
            dollar_sign_match = Regex::new(&pattern).unwrap();
        }

        let sequence = "$".repeat(size);

        let no_whitespaces = !self.value.chars().all(char::is_whitespace);
        let starts_with_whitespace = self.value.starts_with(char::is_whitespace);
        let ends_with_whitespace = self.value.ends_with(char::is_whitespace);
        let starts_with_dollar = self.value.starts_with('$');
        let ends_with_dollar = self.value.ends_with('$');

        let mut value = self.value.clone();
        if no_whitespaces
            && ((starts_with_whitespace && ends_with_whitespace)
                || starts_with_dollar
                || ends_with_dollar)
        {
            value = format!(" {} ", value);
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

        Ok(format!("{}{}{}", sequence, value, sequence))
    }
}

pub fn peek_inline_math() -> char {
    '$'
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
        mdast::{InlineMath, Node},
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
        let inline_math = InlineMath {
            value: String::from("one\r\ntwo"),
            position: None,
        };
        let node = Node::InlineMath(inline_math.clone());

        assert_eq!(
            inline_math
                .handle(&mut state, &Info::new("", ""), None, &node)
                .unwrap(),
            "$one two$"
        );
    }

    #[test]
    fn preserves_inline_math_when_no_unsafe_break_pattern_matches() {
        let options = Options::default();
        let mut state = State::new(&options);
        state.r#unsafe = vec![Unsafe::new('#', None, None, vec![], vec![], true)];
        let inline_math = InlineMath {
            value: String::from("ordinary math"),
            position: None,
        };
        let node = Node::InlineMath(inline_math.clone());

        assert_eq!(
            inline_math
                .handle(&mut state, &Info::new("", ""), None, &node)
                .unwrap(),
            "$ordinary math$"
        );
    }
}
