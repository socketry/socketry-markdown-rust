use super::*;
use alloc::boxed::Box;

#[test]
fn test_mdx_expression_parse() {
    fn func(_value: &str, _kind: &ExpressionKind) -> Signal {
        Signal::Ok
    }

    let func_accepting = |_a: Box<ExpressionParse>| true;

    assert!(
        matches!(func("a", &ExpressionKind::Expression), Signal::Ok),
        "should expose an `ExpressionParse` type (1)"
    );

    assert!(
        func_accepting(Box::new(func)),
        "should expose an `ExpressionParse` type (2)"
    );
}

#[test]
fn test_mdx_esm_parse() {
    fn func(_value: &str) -> Signal {
        Signal::Ok
    }

    let func_accepting = |_a: Box<EsmParse>| true;

    assert!(
        matches!(func("a"), Signal::Ok),
        "should expose an `EsmParse` type (1)"
    );

    assert!(
        func_accepting(Box::new(func)),
        "should expose an `EsmParse` type (2)"
    );
}
