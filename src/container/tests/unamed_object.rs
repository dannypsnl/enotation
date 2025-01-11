use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use crate::ENotationParser;

use super::super::*;

fn unamed_object(input: &str) -> UnamedObject {
    let mut output = ENotationParser::parse(Rule::unamed_object, input).unwrap();
    UnamedObject::from_pest(&mut output).unwrap()
}

#[test]
fn parse_unamed_object() {
    assert_snapshot!(unamed_object("{1, 2, 3}"), @"{1, 2, 3}");
    // test nested case
    assert_snapshot!(unamed_object("{1, {2, 3}}"), @"{1, {2, 3}}");
}
