use super::*;
use insta::*;

#[test]
fn parse_boolean() {
    let output = ENotation::from_str("#t");
    assert_snapshot!(output, @"#t");

    let output = ENotation::from_str("#f");
    assert_snapshot!(output, @"#f");
}

#[test]
fn parse_integer() {
    let output = ENotation::from_str("123");
    assert_snapshot!(output, @"123");
}

#[test]
fn parse_rational() {
    let output = ENotation::from_str("1/2");
    assert_snapshot!(output, @"1/2");
}

#[test]
fn parse_float() {
    let output = ENotation::from_str("1.23");
    assert_snapshot!(output, @"1.23");
}

#[test]
fn parse_list() {
    let output = ENotation::from_str("(1 2 3)");
    assert_snapshot!(output, @"(1 2 3)");

    // test nested case
    let output = ENotation::from_str("(1 (2 3))");
    assert_snapshot!(output, @"(1 (2 3))");
}

#[test]
fn parse_char() {
    let output = ENotation::from_str("#\\c");
    assert_snapshot!(output, @"#\\c");

    let output = ENotation::from_str("#\\tab");
    assert_snapshot!(output, @"#\\tab");

    let output = ENotation::from_str("#\\/");
    assert_snapshot!(output, @"#\\/");
}

#[test]
fn parse_identifier() {
    let output = ENotation::from_str("abc");
    assert_snapshot!(output, @"abc");

    let output = ENotation::from_str("obscure-name-!$%^&*-_=+<.>/?");
    assert_snapshot!(output, @"obscure-name-!$%^&*-_=+<.>/?");

    let output = ENotation::from_str("世界");
    assert_snapshot!(output, @"世界");

    let output = ENotation::from_str("本好きの下剋上");
    assert_snapshot!(output, @"本好きの下剋上");
}

#[test]
fn parse_string() {
    let output = ENotation::from_str("\"abc\"");
    assert_snapshot!(output, @"\"abc\"");
}

#[test]
fn parse_quoting() {
    let output = ENotation::from_str("'(1 2 3)");
    assert_snapshot!(output, @"'(1 2 3)");

    let output = ENotation::from_str("`(1 2 3)");
    assert_snapshot!(output, @"`(1 2 3)");

    let output = ENotation::from_str("#'(1 2 3)");
    assert_snapshot!(output, @"#'(1 2 3)");
}

#[test]
fn parse_set() {
    let output = ENotation::from_str("#{1 2 3}");
    assert_snapshot!(output, @"#{1 2 3}");

    // empty set
    let output = ENotation::from_str("#{}");
    assert_snapshot!(output, @"#{}");
}

#[test]
fn parse_object() {
    let output = ENotation::from_str("{a: 2, b: 3}");
    assert_snapshot!(output, @"{a: 2, b: 3}");

    // unnamed object
    let output = ENotation::from_str("{1, 2, 3}");
    assert_snapshot!(output, @"{1, 2, 3}");

    // empty object
    let output = ENotation::from_str("{}");
    assert_snapshot!(output, @"{}");
}

#[test]
fn parse_comment() {
    let output = ENotationParser::parse(Rule::COMMENT, "; this is a comment")
        .unwrap()
        .peek();
    assert!(output.is_none());

    let output = ENotationParser::parse(Rule::COMMENT, "#;1").unwrap().peek();
    assert!(output.is_none());

    let output = ENotationParser::parse(Rule::COMMENT, "#;(1 2 3)")
        .unwrap()
        .peek();
    assert!(output.is_none());
}
