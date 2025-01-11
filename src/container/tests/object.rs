use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use crate::ENotationParser;

use super::super::*;

fn object_pair(input: &str) -> ObjectPair {
    let mut output = ENotationParser::parse(Rule::object_pair, input).unwrap();
    ObjectPair::from_pest(&mut output).unwrap()
}

fn object(input: &str) -> Object {
    let mut output = ENotationParser::parse(Rule::object, input).unwrap();
    Object::from_pest(&mut output).unwrap()
}

#[test]
fn parse_object_pair() {
    assert_snapshot!(object_pair("a: 1"), @"a: 1");
}

#[test]
fn parse_object() {
    assert_snapshot!(object("{a: 1, b: 2, c: 3}"), @"{a: 1, b: 2, c: 3}");
    // test nested case
    assert_snapshot!(object("{a: 1, b: {c: 2, d: 3}}"), @"{a: 1, b: {c: 2, d: 3}}");
}
