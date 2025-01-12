use from_pest::FromPest;
use pest::Parser;

use crate::{ENotationParser, Rule};

use super::*;
use insta::assert_snapshot;

fn boolean(input: &str) -> Boolean {
    let mut output = ENotationParser::parse(Rule::boolean, input).unwrap();
    Boolean::from_pest(&mut output).unwrap()
}
fn integer(input: &str) -> Integer {
    let mut output = ENotationParser::parse(Rule::int, input).unwrap();
    Integer::from_pest(&mut output).unwrap()
}
fn rational(input: &str) -> Rational {
    let mut output = ENotationParser::parse(Rule::rational, input).unwrap();
    Rational::from_pest(&mut output).unwrap()
}
fn float(input: &str) -> Float {
    let mut output = ENotationParser::parse(Rule::float, input).unwrap();
    Float::from_pest(&mut output).unwrap()
}
fn char(input: &str) -> Char {
    let mut output = ENotationParser::parse(Rule::char, input).unwrap();
    Char::from_pest(&mut output).unwrap()
}
fn identifier(input: &str) -> Identifier {
    let mut output = ENotationParser::parse(Rule::identifier, input).unwrap();
    Identifier::from_pest(&mut output).unwrap()
}
fn string(input: &str) -> String_ {
    let mut output = ENotationParser::parse(Rule::string, input).unwrap();
    String_::from_pest(&mut output).unwrap()
}
fn literal(input: &str) -> Literal {
    let mut output = ENotationParser::parse(Rule::literal, input).unwrap();
    Literal::from_pest(&mut output).unwrap()
}

#[test]
fn parse_boolean() {
    assert_snapshot!(boolean("#t"), @"#t");
    assert_snapshot!(boolean("#f"), @"#f");
}

#[test]
fn parse_integer() {
    assert_snapshot!(integer("123"), @"123");
    assert_snapshot!(integer("-1"), @"-1");
    assert_snapshot!(integer("-10"), @"-10");
    assert_snapshot!(integer("0"), @"0");
    assert_snapshot!(integer("+0"), @"0");
    assert_snapshot!(integer("-0"), @"0");
}

#[test]
fn parse_rational() {
    assert_snapshot!(rational("1/2"), @"1/2");
}

#[test]
fn parse_float() {
    assert_snapshot!(float("1.23"), @"1.23");
}

#[test]
fn parse_char() {
    assert_snapshot!(char("#\\c"), @"#\\c");
    assert_snapshot!(char("#\\tab"), @"#\\tab");
    assert_snapshot!(char("#\\/"), @"#\\/");
}

#[test]
fn parse_identifier() {
    assert_snapshot!(identifier("abc"), @"abc");
    assert_snapshot!(identifier("obscure-name-!$%^&*-_=+<.>/?"), @"obscure-name-!$%^&*-_=+<.>/?");
    assert_snapshot!(identifier("世界"), @"世界");
    assert_snapshot!(identifier("本好きの下剋上"), @"本好きの下剋上");
}

#[test]
fn parse_string() {
    assert_snapshot!(string("\"abc\""), @"\"abc\"");
}

#[test]
fn parse_literal() {
    assert_snapshot!(literal("#t"), @"#t");
    assert_snapshot!(literal("123"), @"123");
    assert_snapshot!(literal("1/2"), @"1/2");
    assert_snapshot!(literal("1.23"), @"1.23");
    assert_snapshot!(literal("#\\c"), @"#\\c");
    assert_snapshot!(literal("\"abc\""), @"\"abc\"");
}