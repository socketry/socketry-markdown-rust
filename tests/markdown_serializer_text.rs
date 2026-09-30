use pretty_assertions::assert_eq;
use socketry_markdown::markdown::to_markdown as to;
use socketry_markdown::mdast::{Node, Text};

#[test]
fn text() {
    assert_eq!(
        to(&Node::Text(Text {
            value: String::new(),
            position: None,
        }))
        .unwrap(),
        "",
        "should support an empty text"
    );

    assert_eq!(
        to(&Node::Text(Text {
            value: String::from("a\nb"),
            position: None,
        }))
        .unwrap(),
        "a\nb\n",
        "should support text"
    );
}
