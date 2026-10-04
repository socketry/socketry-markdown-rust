use super::*;
use crate::util::mdx::{ExpressionKind, Signal};
use alloc::format;

#[test]
fn test_constructs() {
    Constructs::default();
    Constructs::gfm();
    Constructs::mdx();

    let constructs = Constructs::default();
    assert!(constructs.attention, "should default to `CommonMark` (1)");
    assert!(
        !constructs.gfm_autolink_literal,
        "should default to `CommonMark` (2)"
    );
    assert!(
        !constructs.mdx_jsx_flow,
        "should default to `CommonMark` (3)"
    );
    assert!(
        !constructs.frontmatter,
        "should default to `CommonMark` (4)"
    );

    let constructs = Constructs::gfm();
    assert!(constructs.attention, "should support `gfm` shortcut (1)");
    assert!(
        constructs.gfm_autolink_literal,
        "should support `gfm` shortcut (2)"
    );
    assert!(
        !constructs.mdx_jsx_flow,
        "should support `gfm` shortcut (3)"
    );
    assert!(!constructs.frontmatter, "should support `gfm` shortcut (4)");

    let constructs = Constructs::mdx();
    assert!(constructs.attention, "should support `gfm` shortcut (1)");
    assert!(
        !constructs.gfm_autolink_literal,
        "should support `mdx` shortcut (2)"
    );
    assert!(constructs.mdx_jsx_flow, "should support `mdx` shortcut (3)");
    assert!(!constructs.frontmatter, "should support `mdx` shortcut (4)");
}

#[test]
fn test_parse_options() {
    ParseOptions::default();
    ParseOptions::gfm();
    ParseOptions::mdx();

    let options = ParseOptions::default();
    assert!(
        options.constructs.attention,
        "should default to `CommonMark` (1)"
    );
    assert!(
        !options.constructs.gfm_autolink_literal,
        "should default to `CommonMark` (2)"
    );
    assert!(
        !options.constructs.mdx_jsx_flow,
        "should default to `CommonMark` (3)"
    );

    let options = ParseOptions::gfm();
    assert!(
        options.constructs.attention,
        "should support `gfm` shortcut (1)"
    );
    assert!(
        options.constructs.gfm_autolink_literal,
        "should support `gfm` shortcut (2)"
    );
    assert!(
        !options.constructs.mdx_jsx_flow,
        "should support `gfm` shortcut (3)"
    );

    let options = ParseOptions::mdx();
    assert!(
        options.constructs.attention,
        "should support `mdx` shortcut (1)"
    );
    assert!(
        !options.constructs.gfm_autolink_literal,
        "should support `mdx` shortcut (2)"
    );
    assert!(
        options.constructs.mdx_jsx_flow,
        "should support `mdx` shortcut (3)"
    );

    assert_eq!(
            format!("{:?}", ParseOptions::default()),
            "ParseOptions { constructs: Constructs { attention: true, autolink: true, block_quote: true, character_escape: true, character_reference: true, code_indented: true, code_fenced: true, code_text: true, definition: true, frontmatter: false, gfm_autolink_literal: false, gfm_footnote_definition: false, gfm_label_start_footnote: false, gfm_strikethrough: false, gfm_table: false, gfm_task_list_item: false, hard_break_escape: true, hard_break_trailing: true, heading_atx: true, heading_setext: true, html_flow: true, html_text: true, label_start_image: true, label_start_link: true, label_end: true, list_item: true, math_flow: false, math_text: false, mdx_esm: false, mdx_expression_flow: false, mdx_expression_text: false, mdx_jsx_flow: false, mdx_jsx_text: false, thematic_break: true }, inline_code_info: false, html_block_blank_lines: false, html_tag_namespaces: false, gfm_strikethrough_single_tilde: true, math_text_single_dollar: true, mdx_expression_parse: None, mdx_esm_parse: None }",
            "should support `Debug` trait"
        );
    let options = ParseOptions {
        mdx_esm_parse: Some(Box::new(|_value| Signal::Ok)),
        mdx_expression_parse: Some(Box::new(|_value, _kind| Signal::Ok)),
        ..Default::default()
    };
    assert_eq!(
            format!("{options:?}"),
            "ParseOptions { constructs: Constructs { attention: true, autolink: true, block_quote: true, character_escape: true, character_reference: true, code_indented: true, code_fenced: true, code_text: true, definition: true, frontmatter: false, gfm_autolink_literal: false, gfm_footnote_definition: false, gfm_label_start_footnote: false, gfm_strikethrough: false, gfm_table: false, gfm_task_list_item: false, hard_break_escape: true, hard_break_trailing: true, heading_atx: true, heading_setext: true, html_flow: true, html_text: true, label_start_image: true, label_start_link: true, label_end: true, list_item: true, math_flow: false, math_text: false, mdx_esm: false, mdx_expression_flow: false, mdx_expression_text: false, mdx_jsx_flow: false, mdx_jsx_text: false, thematic_break: true }, inline_code_info: false, html_block_blank_lines: false, html_tag_namespaces: false, gfm_strikethrough_single_tilde: true, math_text_single_dollar: true, mdx_expression_parse: Some(\"[Function]\"), mdx_esm_parse: Some(\"[Function]\") }",
            "should support `Debug` trait on mdx functions"
        );
    assert!(matches!(
        options.mdx_esm_parse.as_ref().unwrap()("export const value = 1"),
        Signal::Ok
    ));
    assert!(matches!(
        options.mdx_expression_parse.as_ref().unwrap()("value", &ExpressionKind::Expression),
        Signal::Ok
    ));
}

#[test]
fn test_compile_options() {
    CompileOptions::default();
    CompileOptions::gfm();

    let options = CompileOptions::default();
    assert!(
        !options.allow_dangerous_html,
        "should default to safe `CommonMark` (1)"
    );
    assert!(
        !options.gfm_tagfilter,
        "should default to safe `CommonMark` (2)"
    );

    let options = CompileOptions::gfm();
    assert!(
        !options.allow_dangerous_html,
        "should support safe `gfm` shortcut (1)"
    );
    assert!(
        options.gfm_tagfilter,
        "should support safe `gfm` shortcut (1)"
    );
}

#[test]
fn test_options() {
    Options::default();

    let options = Options::default();
    assert!(
        options.parse.constructs.attention,
        "should default to safe `CommonMark` (1)"
    );
    assert!(
        !options.parse.constructs.gfm_autolink_literal,
        "should default to safe `CommonMark` (2)"
    );
    assert!(
        !options.parse.constructs.mdx_jsx_flow,
        "should default to safe `CommonMark` (3)"
    );
    assert!(
        !options.compile.allow_dangerous_html,
        "should default to safe `CommonMark` (4)"
    );

    let options = Options::gfm();
    assert!(
        options.parse.constructs.attention,
        "should support safe `gfm` shortcut (1)"
    );
    assert!(
        options.parse.constructs.gfm_autolink_literal,
        "should support safe `gfm` shortcut (2)"
    );
    assert!(
        !options.parse.constructs.mdx_jsx_flow,
        "should support safe `gfm` shortcut (3)"
    );
    assert!(
        !options.compile.allow_dangerous_html,
        "should support safe `gfm` shortcut (4)"
    );
}
