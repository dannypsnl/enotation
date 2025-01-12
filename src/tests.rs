use crate::{EFile, ENotation, ENotationParser, Rule};
use from_pest::FromPest;
use pest::Parser;

use insta::*;

fn notation(input: &str) -> ENotation {
    let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
    ENotation::from_pest(&mut output).unwrap()
}
fn all(input: &str) -> EFile {
    let mut output = ENotationParser::parse(Rule::file, input).expect("????");
    match EFile::from_pest(&mut output) {
        Ok(f) => return f,
        Err(err) => panic!("{}", err),
    }
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
fn parse_all() {
    assert_snapshot!(all("
    ; a list
    (1 2 3)
    "), @"(1 2 3)");
    // assert_snapshot!(all("
    // (define x : i32 1)

    // (: f : int -> int)
    // (define (f x)
    //   (add1 x))
    // "),
    //       @"");
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
