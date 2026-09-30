use pretty_assertions::assert_eq;
use socketry_markdown::{message, to_mdast, CompileOptions, HTMLRenderer, ParseOptions};

fn render(
    source: &str,
    parse_options: &ParseOptions,
    compile_options: CompileOptions,
) -> Result<String, message::Message> {
    let tree = to_mdast(source, parse_options)?;
    let mut renderer = HTMLRenderer::with_options(compile_options);
    Ok(tree.render_with(&mut renderer))
}

#[test]
fn renders_ast_nodes_and_heading_ids() -> Result<(), message::Message> {
    assert_eq!(
        render(
            "# Intro\n\nHello, *world* & `code`.",
            &ParseOptions::default(),
            CompileOptions {
                heading_ids: true,
                ..CompileOptions::default()
            },
        )?,
        "<h1 id=\"intro\">Intro</h1>\n<p>Hello, <em>world</em> &amp; <code>code</code>.</p>"
    );

    assert_eq!(
        render(
            "```rust\nlet value = \"<tag>\";\n```",
            &ParseOptions::default(),
            CompileOptions::default(),
        )?,
        "<pre><code class=\"language-rust\">let value = &quot;&lt;tag&gt;&quot;;\n</code></pre>"
    );

    Ok(())
}

#[test]
fn applies_html_and_url_safety_options() -> Result<(), message::Message> {
    let source = "<b>safe text</b>\n\n[link](javascript:alert(1))";

    assert_eq!(
        render(source, &ParseOptions::default(), CompileOptions::default())?,
        "<p>&lt;b&gt;safe text&lt;/b&gt;</p>\n<p><a href=\"\">link</a></p>"
    );

    assert_eq!(
        render(
            source,
            &ParseOptions::default(),
            CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::default()
            },
        )?,
        "<p><b>safe text</b></p>\n<p><a href=\"javascript:alert(1)\">link</a></p>"
    );

    Ok(())
}

#[test]
fn renders_references_and_rich_inline_code() -> Result<(), message::Message> {
    assert_eq!(
        render(
            "[hello][id]\n\n[id]: /guide \"Guide\"",
            &ParseOptions::default(),
            CompileOptions::default(),
        )?,
        "<p><a href=\"/guide\" title=\"Guide\">hello</a></p>"
    );

    assert_eq!(
        render(
            "Use ruby:`puts 1`.",
            &ParseOptions {
                inline_code_info: true,
                ..ParseOptions::default()
            },
            CompileOptions::default(),
        )?,
        "<p>Use <code class=\"language-ruby\">puts 1</code>.</p>"
    );

    Ok(())
}

#[test]
fn renders_gfm_tables_and_task_lists() -> Result<(), message::Message> {
    assert_eq!(
        render(
            "- [x] done\n\n| A | B |\n| :- | -: |\n| one | two |",
            &ParseOptions::gfm(),
            CompileOptions::default(),
        )?,
        "<ul>\n<li><input type=\"checkbox\" checked=\"\" disabled=\"\" /> done</li>\n</ul>\n<table>\n<thead>\n<tr><th align=\"left\">A</th><th align=\"right\">B</th></tr>\n</thead>\n<tbody>\n<tr><td align=\"left\">one</td><td align=\"right\">two</td></tr>\n</tbody>\n</table>"
    );

    Ok(())
}

#[test]
fn renders_gfm_footnotes() -> Result<(), message::Message> {
    assert_eq!(
        render(
            "Call.[^note]\n\n[^note]: details",
            &ParseOptions::gfm(),
            CompileOptions::default(),
        )?,
        "<p>Call.<sup><a href=\"#user-content-fn-note\" id=\"user-content-fnref-note\" data-footnote-ref=\"\" aria-describedby=\"footnote-label\">1</a></sup></p>\n<section data-footnotes=\"\" class=\"footnotes\"><h2 id=\"footnote-label\" class=\"sr-only\">Footnotes</h2>\n<ol>\n<li id=\"user-content-fn-note\">\n<p>details <a href=\"#user-content-fnref-note\" data-footnote-backref=\"\" aria-label=\"Back to content\" class=\"data-footnote-backref\">↩</a></p></li>\n</ol>\n</section>"
    );

    Ok(())
}

#[test]
fn omits_mdx_by_default_and_renders_static_jsx_when_enabled() -> Result<(), message::Message> {
    let source = "<Widget title=\"demo\">hello</Widget>";

    assert_eq!(
        render(source, &ParseOptions::mdx(), CompileOptions::default())?,
        ""
    );
    assert_eq!(
        render(
            source,
            &ParseOptions::mdx(),
            CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
        )?,
        "<p><Widget title=\"demo\">hello</Widget></p>"
    );

    Ok(())
}
