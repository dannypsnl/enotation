use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use crate::ENotationParser;

use super::super::*;

fn vector(input: &str) -> Vector {
    let mut output = ENotationParser::parse(Rule::vector, input).unwrap();
    Vector::from_pest(&mut output).unwrap()
}

#[test]
fn parse_vector() {
    assert_snapshot!(vector("#(1 2 3)"), @"#(1 2 3)");
    assert_snapshot!(vector("#[1 2 3]"), @"#[1 2 3]");
    // test nested case
    assert_snapshot!(vector("#(1 #(2 3))"), @"#(1 #(2 3))");
}
