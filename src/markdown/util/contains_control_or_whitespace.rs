// Released under the MIT License.
// Copyright, 2024, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

pub fn contains_control_or_whitespace(value: &str) -> bool {
    value.chars().any(|c| c.is_whitespace() || c.is_control())
}
