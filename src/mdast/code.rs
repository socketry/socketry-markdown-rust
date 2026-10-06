// Released under the MIT License.
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
