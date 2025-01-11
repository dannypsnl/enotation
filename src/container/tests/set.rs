use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use crate::ENotationParser;

use super::super::*;

fn set(input: &str) -> Set {
    let mut output = ENotationParser::parse(Rule::set, input).unwrap();
    Set::from_pest(&mut output).unwrap()
}

#[test]
fn parse_set() {
    assert_snapshot!(set("#{1 2 3}"), @"#{1 2 3}");
    // test nested case
    assert_snapshot!(set("#{1 #{2 3}}"), @"#{1 #{2 3}}");
}
