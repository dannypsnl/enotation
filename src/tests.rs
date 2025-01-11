use super::*;
use from_pest::FromPest;
use insta::*;
use pest::Parser;

impl ENotation {
    fn boolean(input: &str) -> Boolean {
        let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
        Boolean::from_pest(&mut output).unwrap()
    }
}

#[test]
fn parse_boolean() {
    assert_snapshot!(ENotation::boolean("#t"), @"#t");
    assert_snapshot!(ENotation::boolean("#f"), @"#f");
}

// #[test]
// fn parse_integer() {
//     assert_snapshot!(ENotation::from_str("123"), @"123");
//     assert_snapshot!(ENotation::from_str("-1"), @"-1");
//     assert_snapshot!(ENotation::from_str("-10"), @"-10");
//     assert_snapshot!(ENotation::from_str("0"), @"0");
//     assert_snapshot!(ENotation::from_str("+0"), @"0");
//     assert_snapshot!(ENotation::from_str("-0"), @"0");
// }

// #[test]
// fn parse_rational() {
//     assert_snapshot!(ENotation::from_str("1/2"), @"1/2");
// }

// #[test]
// fn parse_float() {
//     assert_snapshot!(ENotation::from_str("1.23"), @"1.23");
// }

// #[test]
// fn parse_list() {
//     assert_snapshot!(ENotation::from_str("(1 2 3)"), @"(1 2 3)");
//     // test nested case
//     assert_snapshot!(ENotation::from_str("(1 (2 3))"), @"(1 (2 3))");
// }

// #[test]
// fn parse_char() {
//     assert_snapshot!(ENotation::from_str("#\\c"), @"#\\c");
//     assert_snapshot!(ENotation::from_str("#\\tab"), @"#\\tab");
//     assert_snapshot!(ENotation::from_str("#\\/"), @"#\\/");
// }

// #[test]
// fn parse_identifier() {
//     assert_snapshot!(ENotation::from_str("abc"), @"abc");
//     assert_snapshot!(ENotation::from_str("obscure-name-!$%^&*-_=+<.>/?"), @"obscure-name-!$%^&*-_=+<.>/?");
//     assert_snapshot!(ENotation::from_str("世界"), @"世界");
//     assert_snapshot!(ENotation::from_str("本好きの下剋上"), @"本好きの下剋上");
// }

// #[test]
// fn parse_string() {
//     assert_snapshot!(ENotation::from_str("\"abc\""), @"\"abc\"");
// }

// #[test]
// fn parse_quoting() {
//     // quote
//     assert_snapshot!(ENotation::from_str("'(1 2 3)"), @"'(1 2 3)");
//     // quasiquote
//     assert_snapshot!(ENotation::from_str("`(1 2 3)"), @"`(1 2 3)");
//     // syntax
//     assert_snapshot!(ENotation::from_str("#'(1 2 3)"), @"#'(1 2 3)");
// }

// #[test]
// fn parse_set() {
//     // set
//     assert_snapshot!(ENotation::from_str("#{1 2 3}"), @"#{1 2 3}");
//     // empty set
//     assert_snapshot!(ENotation::from_str("#{}"), @"#{}");
// }

// #[test]
// fn parse_object() {
//     assert_snapshot!(ENotation::from_str("{a: 2, b: 3}"), @"{a: 2, b: 3}");
//     // unnamed object
//     assert_snapshot!(ENotation::from_str("{1, 2, 3}"), @"{1, 2, 3}");
//     // empty object
//     assert_snapshot!(ENotation::from_str("{}"), @"{}");
// }

// #[test]
// fn parse_comment() {
//     let output = ENotationParser::parse(Rule::COMMENT, "; this is a comment")
//         .unwrap()
//         .peek();
//     assert_debug_snapshot!(output, @"None");

//     let output = ENotationParser::parse(Rule::COMMENT, "#;1").unwrap().peek();
//     assert_debug_snapshot!(output, @"None");

//     // let output = ENotationParser::parse(Rule::COMMENT, "#;(1 2 3)")
//     //     .unwrap()
//     //     .peek();
//     // assert_debug_snapshot!(output, @"None");
// }

// // #[test]
// // fn parse_all() {
// //     let results = parse_str(
// //         "
// // (a b c)

// // (define x : i8 1)
// // ",
// //     )
// //     .unwrap();

// //     assert_debug_snapshot!(results);
// // }
