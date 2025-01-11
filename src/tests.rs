use super::*;
use from_pest::FromPest;
use insta::*;
use pest::Parser;

impl ENotation {
    fn boolean(input: &str) -> Boolean {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Boolean::from_pest(&mut output).unwrap()
    }
    fn object(input: &str) -> Object {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Object::from_pest(&mut output).unwrap()
    }
    fn set(input: &str) -> Set {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Set::from_pest(&mut output).unwrap()
    }
    fn integer(input: &str) -> Integer {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Integer::from_pest(&mut output).unwrap()
    }
    fn rational(input: &str) -> Rational {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Rational::from_pest(&mut output).unwrap()
    }
    fn float(input: &str) -> Float {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Float::from_pest(&mut output).unwrap()
    }
    fn char(input: &str) -> Char {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Char::from_pest(&mut output).unwrap()
    }
    fn identifier(input: &str) -> Identifier {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Identifier::from_pest(&mut output).unwrap()
    }
    fn string(input: &str) -> String_ {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        String_::from_pest(&mut output).unwrap()
    }
    fn list(input: &str) -> List {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        List::from_pest(&mut output).unwrap()
    }
    fn quote(input: &str) -> Quote {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Quote::from_pest(&mut output).unwrap()
    }
    fn quasiquote(input: &str) -> QuasiQuote {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        QuasiQuote::from_pest(&mut output).unwrap()
    }
    fn syntax(input: &str) -> Syntax {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Syntax::from_pest(&mut output).unwrap()
    }
}

#[test]
fn parse_boolean() {
    assert_snapshot!(ENotation::boolean("#t"), @"#t");
    assert_snapshot!(ENotation::boolean("#f"), @"#f");
}

#[test]
fn parse_integer() {
    assert_snapshot!(ENotation::integer("123"), @"123");
    assert_snapshot!(ENotation::integer("-1"), @"-1");
    assert_snapshot!(ENotation::integer("-10"), @"-10");
    assert_snapshot!(ENotation::integer("0"), @"0");
    assert_snapshot!(ENotation::integer("+0"), @"0");
    assert_snapshot!(ENotation::integer("-0"), @"0");
}

#[test]
fn parse_rational() {
    assert_snapshot!(ENotation::rational("1/2"), @"1/2");
}

#[test]
fn parse_float() {
    assert_snapshot!(ENotation::float("1.23"), @"1.23");
}

#[test]
fn parse_char() {
    assert_snapshot!(ENotation::char("#\\c"), @"#\\c");
    assert_snapshot!(ENotation::char("#\\tab"), @"#\\tab");
    assert_snapshot!(ENotation::char("#\\/"), @"#\\/");
}

#[test]
fn parse_identifier() {
    assert_snapshot!(ENotation::identifier("abc"), @"abc");
    assert_snapshot!(ENotation::identifier("obscure-name-!$%^&*-_=+<.>/?"), @"obscure-name-!$%^&*-_=+<.>/?");
    assert_snapshot!(ENotation::identifier("世界"), @"世界");
    assert_snapshot!(ENotation::identifier("本好きの下剋上"), @"本好きの下剋上");
}

#[test]
fn parse_string() {
    assert_snapshot!(ENotation::string("\"abc\""), @"\"abc\"");
}

#[test]
fn parse_list() {
    assert_snapshot!(ENotation::list("(1 2 3)"), @"(1 2 3)");
    // test nested case
    assert_snapshot!(ENotation::list("(1 (2 3))"), @"(1 (2 3))");
}

#[test]
fn parse_quoting() {
    // quote
    assert_snapshot!(ENotation::quote("'(1 2 3)"), @"'(1 2 3)");
    // quasiquote
    assert_snapshot!(ENotation::quasiquote("`(1 2 3)"), @"`(1 2 3)");
    // syntax
    assert_snapshot!(ENotation::syntax("#'(1 2 3)"), @"#'(1 2 3)");
}

#[test]
fn parse_set() {
    // set
    assert_snapshot!(ENotation::set("#{1 2 3}"), @"#{1 2 3}");
    // empty set
    assert_snapshot!(ENotation::set("#{}"), @"#{}");
}

#[test]
fn parse_object() {
    assert_snapshot!(ENotation::object("{a: 2, b: 3}"), @"{a: 2, b: 3}");
    // unnamed object
    assert_snapshot!(ENotation::object("{1, 2, 3}"), @"{1, 2, 3}");
    // empty object
    assert_snapshot!(ENotation::object("{}"), @"{}");
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
