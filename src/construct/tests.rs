use crate::{
    construct::{
        content, flow, frontmatter, gfm_autolink_literal, html_flow, html_text, label_end,
        partial_data, partial_mdx_jsx, raw_flow,
    },
    event::Point,
    parser::{parse, ParseState},
    state::{Name as StateName, State},
    tokenizer::Tokenizer,
    Constructs, ParseOptions,
};
use alloc::{boxed::Box, string::String};

fn with_tokenizer<T>(
    source: &str,
    options: &ParseOptions,
    current: Option<u8>,
    run: impl FnOnce(&mut Tokenizer<'_>) -> T,
) -> T {
    let (_events, parse_state): (_, ParseState<'_>) = parse(source, options).expect("parse input");
    let mut tokenizer = Tokenizer::new(
        Point {
            line: 1,
            column: 1,
            index: 0,
            vs: 0,
        },
        &parse_state,
    );
    tokenizer.current = current;
    run(&mut tokenizer)
}

#[test]
fn partial_data_retries_when_there_is_no_initial_marker() {
    with_tokenizer("", &ParseOptions::default(), None, |tokenizer| {
        assert_eq!(
            partial_data::start(tokenizer),
            State::Retry(StateName::DataAtBreak)
        );
    });
}

#[test]
fn content_resolver_subtokenizes_even_an_empty_event_stream() {
    with_tokenizer("", &ParseOptions::default(), None, |tokenizer| {
        assert!(content::resolve(tokenizer).done);
    });
}

#[test]
fn raw_flow_info_rejects_its_own_fence_marker() {
    for marker in *b"`$" {
        with_tokenizer("x", &ParseOptions::default(), Some(marker), |tokenizer| {
            tokenizer.tokenize_state.marker = marker;

            assert_eq!(raw_flow::info(tokenizer), State::Nok);
            assert_eq!(tokenizer.tokenize_state.marker, 0);
            assert!(!tokenizer.concrete);
        });
    }
}

#[test]
fn gfm_autolink_literal_accepts_uppercase_protocol_and_www_prefixes() {
    let options = ParseOptions {
        constructs: Constructs {
            gfm_autolink_literal: true,
            ..Constructs::default()
        },
        ..ParseOptions::default()
    };

    with_tokenizer("H", &options, Some(b'H'), |tokenizer| {
        assert_eq!(
            gfm_autolink_literal::protocol_start(tokenizer),
            State::Retry(StateName::GfmAutolinkLiteralProtocolPrefixInside)
        );
    });

    with_tokenizer("W", &options, Some(b'W'), |tokenizer| {
        assert_eq!(
            gfm_autolink_literal::www_start(tokenizer),
            State::Retry(StateName::GfmAutolinkLiteralWwwPrefixInside)
        );
    });
}

#[test]
fn accepts_eof_and_line_endings_after_content_definitions_and_frontmatter() {
    with_tokenizer("", &ParseOptions::default(), None, |tokenizer| {
        assert_eq!(content::definition_after(tokenizer), State::Ok);
    });

    parse("[label]: /url", &ParseOptions::default()).expect("parses a definition at EOF");
    parse("[label]: /url\n", &ParseOptions::default())
        .expect("parses a definition followed by a line ending");

    let frontmatter_options = ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Constructs::default()
        },
        ..ParseOptions::default()
    };
    parse("---\ntitle: Example\n---", &frontmatter_options).expect("parses frontmatter at EOF");
    parse("---\ntitle: Example\n---\n", &frontmatter_options)
        .expect("parses frontmatter followed by a line ending");
}

#[test]
#[should_panic(expected = "expected eol/eof after closing fence")]
fn frontmatter_after_rejects_other_bytes() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        frontmatter::after,
    );
}

#[test]
#[should_panic(expected = "assertion failed: matches!(tokenizer.current, None | Some(b'\\n'))")]
fn content_definition_after_rejects_other_bytes() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        content::definition_after,
    );
}

#[test]
fn reports_mdx_expression_parse_errors() {
    let options = ParseOptions {
        constructs: Constructs {
            mdx_expression_text: true,
            ..Constructs::default()
        },
        mdx_expression_parse: Some(Box::new(|_, _| {
            crate::util::mdx::Signal::Error(
                String::from("invalid expression"),
                0,
                Box::new(String::from("test")),
                Box::new(String::from("invalid-expression")),
            )
        })),
        ..ParseOptions::default()
    };
    let error = parse("before {value} after", &options)
        .expect_err("propagate expression parser errors through content resolution");

    assert_eq!(error.reason, "invalid expression");
    assert_eq!(error.source.as_str(), "test");
    assert_eq!(error.rule_id.as_str(), "invalid-expression");
}

#[test]
fn reports_mdx_esm_parse_errors_during_content_resolution() {
    let options = ParseOptions {
        constructs: Constructs {
            mdx_esm: true,
            ..Constructs::default()
        },
        mdx_esm_parse: Some(Box::new(|_| {
            crate::util::mdx::Signal::Error(
                String::from("invalid ESM"),
                0,
                Box::new(String::from("test")),
                Box::new(String::from("invalid-esm")),
            )
        })),
        ..ParseOptions::default()
    };

    let error = parse("export const value = 1", &options)
        .expect_err("report errors from the configured MDX ESM parser");
    assert_eq!(error.reason, "invalid ESM");
    assert_eq!(error.source.as_str(), "test");
    assert_eq!(error.rule_id.as_str(), "invalid-esm");
}

#[test]
fn reports_mdx_expression_errors_without_a_matching_source_position() {
    let options = ParseOptions {
        constructs: Constructs {
            mdx_expression_text: true,
            ..Constructs::default()
        },
        mdx_expression_parse: Some(Box::new(|_, _| {
            crate::util::mdx::Signal::Error(
                String::from("invalid expression"),
                1000,
                Box::new(String::from("test")),
                Box::new(String::from("invalid-expression")),
            )
        })),
        ..ParseOptions::default()
    };

    assert!(parse("{value}", &options).is_err());
}

#[test]
fn reports_mdx_flow_expression_errors_inside_block_quotes() {
    let options = ParseOptions {
        constructs: Constructs {
            mdx_expression_flow: true,
            ..Constructs::default()
        },
        mdx_expression_parse: Some(Box::new(|_, _| {
            crate::util::mdx::Signal::Error(
                String::from("invalid expression"),
                0,
                Box::new(String::from("test")),
                Box::new(String::from("invalid-expression")),
            )
        })),
        ..ParseOptions::default()
    };

    let error = parse("> {value}", &options).expect_err("report nested MDX parse errors");

    assert_eq!(error.reason, "invalid expression");
    assert_eq!(error.source.as_str(), "test");
    assert_eq!(error.rule_id.as_str(), "invalid-expression");
}

#[test]
#[should_panic(expected = "unexpected eol/eof")]
fn content_chunk_rejects_an_empty_start() {
    with_tokenizer("", &ParseOptions::default(), None, content::chunk_start);
}

#[test]
#[should_panic(expected = "expected eol/eof")]
fn blank_line_after_rejects_non_line_endings() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        flow::blank_line_after,
    );
}

#[test]
#[should_panic(expected = "expected eof/eol")]
fn frontmatter_content_end_rejects_other_bytes() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        frontmatter::content_end,
    );
}

#[test]
#[should_panic(expected = "expected eol")]
fn html_flow_continuation_requires_a_line_ending() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        html_flow::continuation_start_non_lazy,
    );
}

#[test]
#[should_panic(expected = "expected eol")]
fn html_text_line_ending_requires_a_line_ending() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        html_text::line_ending_before,
    );
}

#[test]
#[should_panic(expected = "expected `(`")]
fn resource_start_requires_an_opening_parenthesis() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        label_end::resource_start,
    );
}

#[test]
#[should_panic(expected = "expected `[`")]
fn full_reference_start_requires_an_opening_bracket() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        label_end::reference_full,
    );
}

#[test]
#[should_panic(expected = "expected `>`")]
fn jsx_tag_end_requires_a_closing_angle_bracket() {
    with_tokenizer(
        "x",
        &ParseOptions::default(),
        Some(b'x'),
        partial_mdx_jsx::tag_end,
    );
}
