use crate::{ENotationParser, Rule};
use pest::Parser;

use insta::*;

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
