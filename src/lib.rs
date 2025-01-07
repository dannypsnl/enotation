#![feature(iter_next_chunk)]
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ENotation {
    Boolean(bool),
    Integer(i64),
    Rational(i64, i64),
    Str(String),
    Identifier(String),
    // (a b c)
    List(Vec<ENotation>),
    Set(Vec<ENotation>),
    Map(Vec<(ENotation, ENotation)>),
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

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}

impl ENotation {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::boolean_true => ENotation::Boolean(true),
            Rule::boolean_false => ENotation::Boolean(false),
            Rule::int => ENotation::Integer(pair.as_str().parse().unwrap()),
            Rule::rational => {
                let mut inner_rules = pair.into_inner();
                let p = inner_rules.next().unwrap().as_str().parse().unwrap();
                let q = inner_rules.next().unwrap().as_str().parse().unwrap();
                ENotation::Rational(p, q)
            }
            Rule::string => ENotation::Str(remove_quotes(pair.as_str())),
            Rule::identifier => ENotation::Identifier(pair.as_str().to_string()),

            Rule::list => ENotation::List(pair.into_inner().map(ENotation::from_pair).collect()),
            Rule::set => ENotation::Set(pair.into_inner().map(ENotation::from_pair).collect()),
            Rule::map => {
                let mut inner_rules = pair.into_inner();
                let mut map_pairs = vec![];
                loop {
                    let Some([key, val]) = inner_rules.next_chunk::<2>().ok() else {
                        break;
                    };
                    map_pairs.push((ENotation::from_pair(key), ENotation::from_pair(val)));
                }
                ENotation::Map(map_pairs)
            }

            Rule::quote => ENotation::Quote(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::quasiquote => ENotation::QuasiQuote(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::unquote => ENotation::Unquote(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::unquote_splicing => ENotation::UnquoteSplicing(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::syntax => ENotation::Syntax(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::quasisyntax => ENotation::QuasiSyntax(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::unsyntax => ENotation::Unsyntax(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
            Rule::unsyntax_splicing => ENotation::UnsyntaxSplicing(Rc::new(ENotation::from_pair(
                pair.into_inner().peek().unwrap(),
            ))),
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

    pub fn from_str(input: &str) -> Self {
        let output = ENotationParser::parse(Rule::notation, input)
            .unwrap()
            .next()
            .unwrap();
        Self::from_pair(output)
    }
}

#[test]
fn parse_boolean() {
    let output = ENotation::from_str("#t");
    assert_eq!(output, ENotation::Boolean(true));

    let output = ENotation::from_str("#f");
    assert_eq!(output, ENotation::Boolean(false));
}

#[test]
fn parse_integer() {
    let output = ENotation::from_str("123");
    assert_eq!(output, ENotation::Integer(123))
}

#[test]
fn parse_rational() {
    let output = ENotation::from_str("1/2");
    assert_eq!(output, ENotation::Rational(1, 2))
}

#[test]
fn parse_list() {
    use ENotation::{Integer as I, List as L};
    let output = ENotation::from_str("(1 2 3)");
    assert_eq!(output, L(vec![I(1), I(2), I(3)]));

    // test nested case
    let output = ENotation::from_str("(1 (2 3))");
    assert_eq!(output, L(vec![I(1), L(vec![I(2), I(3)])]));
}

#[test]
fn parse_identifier() {
    use ENotation::*;
    let output = ENotation::from_str("abc");
    assert_eq!(output, Identifier("abc".to_string()));

    let output = ENotation::from_str("obscure-name-!$%^&*-_=+<.>/?");
    assert_eq!(
        output,
        Identifier("obscure-name-!$%^&*-_=+<.>/?".to_string())
    );

    let output = ENotation::from_str("世界");
    assert_eq!(output, Identifier("世界".to_string()));

    let output = ENotation::from_str("本好きの下剋上");
    assert_eq!(output, Identifier("本好きの下剋上".to_string()));
}

#[test]
fn parse_string() {
    let output = ENotation::from_str("\"abc\"");
    assert_eq!(output, ENotation::Str("abc".to_string()))
}

#[test]
fn parse_quoting() {
    use ENotation::{Integer as I, List as L, QuasiQuote as QQ, Quote as Q, Syntax as S};
    let output = ENotation::from_str("'(1 2 3)");
    assert_eq!(output, Q(L(vec![I(1), I(2), I(3)]).into()));

    let output = ENotation::from_str("`(1 2 3)");
    assert_eq!(output, QQ(L(vec![I(1), I(2), I(3)]).into()));

    let output = ENotation::from_str("#'(1 2 3)");
    assert_eq!(output, S(L(vec![I(1), I(2), I(3)]).into()));
}

#[test]
fn parse_set() {
    use ENotation::{Integer as I, Set as S};
    let output = ENotation::from_str("#{1 2 3}");
    assert_eq!(output, S(vec![I(1), I(2), I(3)]));

    // empty set
    let output = ENotation::from_str("#{}");
    assert_eq!(output, S(vec![]));
}

#[test]
fn parse_map() {
    use ENotation::{Identifier as Id, Integer as I, Map as M, Quote as Q};
    let output = ENotation::from_str("{'a 2, 2 3}");
    assert_eq!(
        output,
        M(vec![(Q(Id("a".to_string()).into()), I(2)), (I(2), I(3))])
    );

    // empty map
    let output = ENotation::from_str("{}");
    assert_eq!(output, M(vec![]));
}
