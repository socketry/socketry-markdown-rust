// Released under the MIT License.
// Copyright, 2022, by Bernhard Berger.
// Copyright, 2022-2025, by Titus Wormer.
// Copyright, 2023, by Kyle McCarthy.
// Copyright, 2023, by Mia.
// Copyright, 2023, by Rafael Bachmann.
// Copyright, 2024, by Harsha Teja Kanna.
// Copyright, 2026, by Samuel Williams.

use super::CodeFence;
use crate::unist::Position;
use alloc::string::String;

/// Code (flow).
///
/// ```markdown
/// > | ~~~
///     ^^^
/// > | a
///     ^
/// > | ~~~
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Code {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub position: Option<Position>,
    // Extra.
    /// The language of computer code being marked up.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub lang: Option<String>,
    /// Custom info relating to the node.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub meta: Option<String>,
    /// Opening fence metadata, or `None` for indented code or an AST without
    /// source fence information. This is independent of source positions.
    /// Markdown serialization chooses its formatting from `MarkdownOptions`.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub fence: Option<CodeFence>,
}
