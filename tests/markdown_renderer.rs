use pretty_assertions::assert_eq;
use socketry_markdown::{
    mdast::Node, to_mdast, Constructs, MarkdownOptions, MarkdownRenderer, ParseOptions,
};

#[test]
fn serializes_a_markdown_document() -> Result<(), socketry_markdown::message::Message> {
    let node = to_mdast(
        "# Hello *world*!\n\nA [link](/url).",
        &ParseOptions::default(),
    )?;

    assert_eq!(node.to_markdown(), "# Hello *world*!\n\nA [link](/url).\n");

    Ok(())
}

#[test]
fn serializes_extracted_fragments() -> Result<(), socketry_markdown::message::Message> {
    let mut node = to_mdast("**hello**", &ParseOptions::default())?;
    let fragment = node.extract_children().expect("root has children");

    assert_eq!(fragment.to_markdown(), "**hello**\n");

    Ok(())
}

#[test]
fn renders_with_custom_markdown_options() -> Result<(), socketry_markdown::message::Message> {
    let node = to_mdast("*hello*", &ParseOptions::default())?;
    let mut renderer = MarkdownRenderer::with_options(MarkdownOptions {
        emphasis: '_',
        ..MarkdownOptions::default()
    });

    assert_eq!(node.render_with(&mut renderer), "_hello_\n");

    Ok(())
}

#[test]
fn serializes_gfm_constructs() -> Result<(), socketry_markdown::message::Message> {
    let node = to_mdast(
        "~~old~~\n\n| A | B |\n| :- | -: |\n| x | y |\n\nCall[^note].\n\n[^note]: Details.",
        &ParseOptions::gfm(),
    )?;

    assert_eq!(
        node.to_markdown(),
        "~~old~~\n\n| A | B |\n| :--- | ---: |\n| x | y |\n\nCall[^note].\n\n[^note]: Details.\n"
    );

    Ok(())
}

#[test]
fn serializes_mdx_constructs() -> Result<(), socketry_markdown::message::Message> {
    let node = to_mdast("a <b>*c*</b> and {value}.", &ParseOptions::mdx())?;

    assert_eq!(node.to_markdown(), "a <b>*c*</b> and {value}.\n");

    Ok(())
}

#[test]
fn serializes_frontmatter() -> Result<(), socketry_markdown::message::Message> {
    let options = ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Constructs::default()
        },
        ..ParseOptions::default()
    };
    let node = to_mdast("---\ntitle: Example\n---\n\nHello", &options)?;

    assert_eq!(node.to_markdown(), "---\ntitle: Example\n---\n\nHello\n");

    Ok(())
}

#[test]
fn serializes_a_node_directly() {
    let node = Node::Text(socketry_markdown::mdast::Text {
        value: "hello".into(),
        position: None,
    });

    assert_eq!(node.to_markdown(), "hello\n");
}
