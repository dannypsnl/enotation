use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use super::{QuasiQuote, Quote};
use crate::{ENotationParser, Rule};

fn quote(input: &str) -> Quote {
    let mut output = ENotationParser::parse(Rule::quote, input).unwrap();
    Quote::from_pest(&mut output).unwrap()
}
fn quasi_quote(input: &str) -> QuasiQuote {
    let mut output = ENotationParser::parse(Rule::quasiquote, input).unwrap();
    QuasiQuote::from_pest(&mut output).unwrap()
}

#[test]
fn parse_quote() {
    assert_snapshot!(quote("'(1 2 3)"), @"'(1 2 3)");
    assert_snapshot!(quote("'1"), @"'1");
    assert_snapshot!(quote("''1"), @"''1");
}

#[test]
fn parse_quasi_quote_and_unquoting() {
    assert_snapshot!(quasi_quote("`(1 2 3)"), @"`(1 2 3)");
    assert_snapshot!(quasi_quote("`(1 2 ,a)"), @"`(1 2 ,a)");
    assert_snapshot!(quasi_quote("`(1 ,@b)"), @"`(1 ,@b)");
}
