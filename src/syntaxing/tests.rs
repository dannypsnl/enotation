use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;

use super::{QuasiSyntax, Syntax};
use crate::{ENotationParser, Rule};

fn syntax(input: &str) -> Syntax {
    let mut output = ENotationParser::parse(Rule::syntax, input).unwrap();
    Syntax::from_pest(&mut output).unwrap()
}
fn quasi_syntax(input: &str) -> QuasiSyntax {
    let mut output = ENotationParser::parse(Rule::quasisyntax, input).unwrap();
    QuasiSyntax::from_pest(&mut output).unwrap()
}

#[test]
fn parse_syntax() {
    assert_snapshot!(syntax("#'(1 2 3)"), @"#'(1 2 3)");
    assert_snapshot!(syntax("#'1"), @"#'1");
    assert_snapshot!(syntax("#'#'1"), @"#'#'1");
}

#[test]
fn parse_quasisyntax() {
    assert_snapshot!(quasi_syntax("#`(1 2 3)"), @"#`(1 2 3)");
    assert_snapshot!(quasi_syntax("#`1"), @"#`1");
    assert_snapshot!(quasi_syntax("#`(1 2 #,a)"), @"#`(1 2 #,a)");
    assert_snapshot!(quasi_syntax("#`(1 2 #,@l 3)"), @"#`(1 2 #,@l 3)");
}
