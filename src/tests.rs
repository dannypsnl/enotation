use crate::{ENotation, ENotationParser, Rule};
use from_pest::FromPest;
use pest::Parser;

use insta::*;

fn notation(input: &str) -> ENotation {
    let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
    ENotation::from_pest(&mut output).unwrap()
}

#[test]
fn parse_random_notation_should_work() {
    assert_snapshot!(notation("1"), @"1");
    assert_snapshot!(notation("#f"), @"#f");
    assert_snapshot!(notation("#\\c"), @r"#\c");
    assert_snapshot!(notation("(1 (2 3))"), @"(1 (2 3))");
    assert_snapshot!(notation("({})"), @"({})");
}

#[test]
fn parse_comment() {
    let output = ENotationParser::parse(Rule::COMMENT, "; this is a comment")
        .unwrap()
        .peek();
    assert_debug_snapshot!(output, @"None");

    let output = ENotationParser::parse(Rule::COMMENT, "#;1").unwrap().peek();
    assert_debug_snapshot!(output, @"None");

    // let output = ENotationParser::parse(Rule::COMMENT, "#;(1 2 3)")
    //     .unwrap()
    //     .peek();
    // assert_debug_snapshot!(output, @"None");
}
