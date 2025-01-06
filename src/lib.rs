use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Debug, PartialEq)]
pub enum ENotation {
    Boolean(bool),
    Integer(num::BigInt),
    Str(String),
    Identifier(String),
    // (a b c)
    List(Vec<ENotation>),
}

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

fn parse_value(pair: Pair<Rule>) -> ENotation {
    match pair.as_rule() {
        Rule::boolean_true => ENotation::Boolean(true),
        Rule::boolean_false => ENotation::Boolean(false),
        Rule::list => ENotation::List(pair.into_inner().map(parse_value).collect()),
        Rule::int => ENotation::Integer(pair.as_str().parse().unwrap()),
        Rule::COMMENT | Rule::WHITESPACE | Rule::single_line_comment | Rule::boolean => {
            unreachable!()
        }
        Rule::paren_list => todo!(),
        Rule::bracket_list => todo!(),
        Rule::notation => todo!(),
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
    assert_eq!(output, ENotation::Integer(num::BigInt::from(123)))
}
