use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use crate::ENotationParser;

use super::super::*;

fn plist(input: &str) -> PList {
    let mut output = ENotationParser::parse(Rule::paren_list, input).unwrap();
    PList::from_pest(&mut output).unwrap()
}
fn blist(input: &str) -> BList {
    let mut output = ENotationParser::parse(Rule::bracket_list, input).unwrap();
    BList::from_pest(&mut output).unwrap()
}
fn list(input: &str) -> List {
    let mut output = ENotationParser::parse(Rule::list, input).unwrap();
    List::from_pest(&mut output).unwrap()
}

#[test]
fn parse_plist() {
    assert_snapshot!(plist("(1 2 3)"), @"(1 2 3)");
    // test nested case
    assert_snapshot!(plist("(1 (2 3))"), @"(1 (2 3))");
}

#[test]
fn parse_blist() {
    assert_snapshot!(blist("[1 2 3]"), @"[1 2 3]");
    // test nested case
    assert_snapshot!(blist("[1 [2 3]]"), @"[1 [2 3]]");
}

#[test]
fn parse_list() {
    assert_snapshot!(list("(1 2 3)"), @"(1 2 3)");
    assert_snapshot!(list("[1 2 3]"), @"[1 2 3]");
    // test nested case
    assert_snapshot!(list("(1 [2 3])"), @"(1 [2 3])");
}
