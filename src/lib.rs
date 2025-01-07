use std::rc::Rc;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Debug, PartialEq)]
pub enum ENotation {
    Boolean(bool),
    Integer(i64),
    Rational(i64, i64),
    Str(String),
    Identifier(String),
    // (a b c)
    List(Vec<ENotation>),
    // '(1 c a)
    Quote(Rc<ENotation>),
    // `(1 c a)
    QuasiQuote(Rc<ENotation>),
    Unquote(Rc<ENotation>),
    UnquoteSplicing(Rc<ENotation>),
    // #'(1 c a)
    Syntax(Rc<ENotation>),
    // #`(1 c a)
    QuasiSyntax(Rc<ENotation>),
    Unsyntax(Rc<ENotation>),
    UnsyntaxSplicing(Rc<ENotation>),
}

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

fn parse_value(pair: Pair<Rule>) -> ENotation {
    match pair.as_rule() {
        Rule::list => ENotation::List(pair.into_inner().map(parse_value).collect()),
        Rule::boolean_true => ENotation::Boolean(true),
        Rule::boolean_false => ENotation::Boolean(false),
        Rule::int => ENotation::Integer(pair.as_str().parse().unwrap()),
        Rule::rational => {
            let mut inner_rules = pair.into_inner();
            let p = inner_rules.next().unwrap().as_str().parse().unwrap();
            let q = inner_rules.next().unwrap().as_str().parse().unwrap();
            ENotation::Rational(p, q)
        }
        Rule::identifier => ENotation::Identifier(pair.as_str().to_string()),
        Rule::quote => ENotation::Quote(Rc::new(parse_value(pair.into_inner().peek().unwrap()))),
        Rule::quasiquote => {
            ENotation::QuasiQuote(Rc::new(parse_value(pair.into_inner().peek().unwrap())))
        }
        Rule::unquote => {
            ENotation::Unquote(Rc::new(parse_value(pair.into_inner().peek().unwrap())))
        }
        Rule::unquote_splicing => {
            ENotation::UnquoteSplicing(Rc::new(parse_value(pair.into_inner().peek().unwrap())))
        }
        Rule::syntax => ENotation::Syntax(Rc::new(parse_value(pair.into_inner().peek().unwrap()))),
        Rule::quasisyntax => {
            ENotation::QuasiSyntax(Rc::new(parse_value(pair.into_inner().peek().unwrap())))
        }
        Rule::unsyntax => {
            ENotation::Unsyntax(Rc::new(parse_value(pair.into_inner().peek().unwrap())))
        }
        Rule::unsyntax_splicing => {
            ENotation::UnsyntaxSplicing(Rc::new(parse_value(pair.into_inner().peek().unwrap())))
        }
        Rule::COMMENT
        | Rule::WHITESPACE
        | Rule::SCHEME_ALPHA
        | Rule::dec_int
        | Rule::single_line_comment
        | Rule::boolean
        | Rule::paren_list
        | Rule::bracket_list
        | Rule::notation => {
            unreachable!()
        }
    }
}

fn parse_str(input: &str) -> ENotation {
    let output = ENotationParser::parse(Rule::notation, input)
        .unwrap()
        .next()
        .unwrap();
    parse_value(output)
}

#[test]
fn parse_boolean() {
    let output = parse_str("#t");
    assert_eq!(output, ENotation::Boolean(true));

    let output = parse_str("#f");
    assert_eq!(output, ENotation::Boolean(false));
}

#[test]
fn parse_integer() {
    let output = parse_str("123");
    assert_eq!(output, ENotation::Integer(123))
}

#[test]
fn parse_rational() {
    let output = parse_str("1/2");
    assert_eq!(output, ENotation::Rational(1, 2))
}

#[test]
fn parse_list() {
    use ENotation::{Integer as I, List as L};
    let output = parse_str("(1 2 3)");
    assert_eq!(output, L(vec![I(1), I(2), I(3)]));

    // test nested case
    let output = parse_str("(1 (2 3))");
    assert_eq!(output, L(vec![I(1), L(vec![I(2), I(3)])]));
}

#[test]
fn parse_identifier() {
    use ENotation::*;
    let output = parse_str("abc");
    assert_eq!(output, Identifier("abc".to_string()));

    let output = parse_str("obscure-name-!$%^&*-_=+<.>/?");
    assert_eq!(
        output,
        Identifier("obscure-name-!$%^&*-_=+<.>/?".to_string())
    );

    let output = parse_str("世界");
    assert_eq!(output, Identifier("世界".to_string()));

    let output = parse_str("本好きの下剋上");
    assert_eq!(output, Identifier("本好きの下剋上".to_string()));
}

#[test]
fn parse_quoting() {
    use ENotation::{Integer as I, List as L, QuasiQuote as QQ, Quote as Q, Syntax as S};
    let output = parse_str("'(1 2 3)");
    assert_eq!(output, Q(L(vec![I(1), I(2), I(3)]).into()));

    let output = parse_str("`(1 2 3)");
    assert_eq!(output, QQ(L(vec![I(1), I(2), I(3)]).into()));

    let output = parse_str("#'(1 2 3)");
    assert_eq!(output, S(L(vec![I(1), I(2), I(3)]).into()));
}
