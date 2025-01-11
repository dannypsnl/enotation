// use super::*;
// use from_pest::FromPest;
// use insta::*;
// use pest::Parser;

// impl ENotation {

//     // fn plist(input: &str) -> PList {
//     //     let mut output = ENotationParser::parse(Rule::paren_list, input).unwrap();
//     //     println!("{:?}", output);
//     //     PList::from_pest(&mut output).unwrap()
//     // }
//     // fn blist(input: &str) -> BList {
//     //     let mut output = ENotationParser::parse(Rule::bracket_list, input).unwrap();
//     //     println!("{:?}", output);
//     //     BList::from_pest(&mut output).unwrap()
//     // }
//     // fn list(input: &str) -> List {
//     //     let mut output = ENotationParser::parse(Rule::list, input).unwrap();
//     //     println!("{:?}", output);
//     //     List::from_pest(&mut output).unwrap()
//     // }
//     // fn set(input: &str) -> Set {
//     //     let mut output = ENotationParser::parse(Rule::set, input).unwrap();
//     //     Set::from_pest(&mut output).unwrap()
//     // }
//     // fn object(input: &str) -> Object {
//     //     let mut output = ENotationParser::parse(Rule::object, input).unwrap();
//     //     Object::from_pest(&mut output).unwrap()
//     // }
//     // fn quote(input: &str) -> Quote {
//     //     let mut output = ENotationParser::parse(Rule::quote, input).unwrap();
//     //     Quote::from_pest(&mut output).unwrap()
//     // }
//     // fn quasiquote(input: &str) -> QuasiQuote {
//     //     let mut output = ENotationParser::parse(Rule::quasiquote, input).unwrap();
//     //     QuasiQuote::from_pest(&mut output).unwrap()
//     // }
//     // fn syntax(input: &str) -> Syntax {
//     //     let mut output = ENotationParser::parse(Rule::syntax, input).unwrap();
//     //     Syntax::from_pest(&mut output).unwrap()
//     // }
// }

// // #[test]
// // fn parse_plist() {
// //     assert_snapshot!(ENotation::plist("(1 2 3)"), @"(1 2 3)");
// //     // test nested case
// //     // assert_snapshot!(ENotation::blist("(1 (2 3))"), @"(1 (2 3))");
// // }

// // #[test]
// // fn parse_list() {
// //     assert_snapshot!(ENotation::list("(1 2 3)"), @"(1 2 3)");
// //     // test nested case
// //     assert_snapshot!(ENotation::list("(1 (2 3))"), @"(1 (2 3))");
// // }

// // #[test]
// // fn parse_quoting() {
// //     // quote
// //     assert_snapshot!(ENotation::quote("'(1 2 3)"), @"'(1 2 3)");
// //     // quasiquote
// //     assert_snapshot!(ENotation::quasiquote("`(1 2 3)"), @"`(1 2 3)");
// //     // syntax
// //     assert_snapshot!(ENotation::syntax("#'(1 2 3)"), @"#'(1 2 3)");
// // }

// // #[test]
// // fn parse_set() {
// //     // set
// //     assert_snapshot!(ENotation::set("#{1 2 3}"), @"#{1 2 3}");
// //     // empty set
// //     assert_snapshot!(ENotation::set("#{}"), @"#{}");
// // }

// // #[test]
// // fn parse_object() {
// //     assert_snapshot!(ENotation::object("{a: 2, b: 3}"), @"{a: 2, b: 3}");
// //     // unnamed object
// //     assert_snapshot!(ENotation::object("{1, 2, 3}"), @"{1, 2, 3}");
// //     // empty object
// //     assert_snapshot!(ENotation::object("{}"), @"{}");
// // }

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
