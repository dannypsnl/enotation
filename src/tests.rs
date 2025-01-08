use super::*;

impl ENotationBody {
    fn as_enotation(&self) -> ENotation {
        ENotation {
            span: Default::default(),
            body: self.clone(),
        }
    }
}
#[test]
fn parse_boolean() {
    use ENotationBody::*;
    let output = ENotation::from_str("#t");
    assert_eq!(output, Boolean(true).as_enotation());

    let output = ENotation::from_str("#f");
    assert_eq!(output, Boolean(false).as_enotation());
}

#[test]
fn parse_integer() {
    use ENotationBody::*;
    let output = ENotation::from_str("123");
    assert_eq!(output, Integer(123).as_enotation())
}

#[test]
fn parse_rational() {
    use ENotationBody::*;
    let output = ENotation::from_str("1/2");
    assert_eq!(output, Rational(1, 2).as_enotation())
}

#[test]
fn parse_float() {
    use ENotationBody::*;
    let output = ENotation::from_str("1.23");
    assert_eq!(output, Float(1.23).as_enotation());
}

#[test]
fn parse_list() {
    use ENotationBody::{Integer as I, List as L};
    let output = ENotation::from_str("(1 2 3)");
    assert_eq!(
        output,
        L(vec![
            I(1).as_enotation(),
            I(2).as_enotation(),
            I(3).as_enotation()
        ])
        .as_enotation()
    );

    // test nested case
    let output = ENotation::from_str("(1 (2 3))");
    assert_eq!(
        output,
        L(vec![
            I(1).as_enotation(),
            L(vec![I(2).as_enotation(), I(3).as_enotation()]).as_enotation()
        ])
        .as_enotation()
    );
}

#[test]
fn parse_char() {
    use ENotationBody::Char as C;
    let output = ENotation::from_str("#\\c");
    assert_eq!(output, C('c').as_enotation());

    let output = ENotation::from_str("#\\tab");
    assert_eq!(output, C('\t').as_enotation());

    let output = ENotation::from_str("#\\/");
    assert_eq!(output, C('/').as_enotation());
}

#[test]
fn parse_identifier() {
    use ENotationBody::*;
    let output = ENotation::from_str("abc");
    assert_eq!(output, Identifier("abc".to_string()).as_enotation());

    let output = ENotation::from_str("obscure-name-!$%^&*-_=+<.>/?");
    assert_eq!(
        output,
        Identifier("obscure-name-!$%^&*-_=+<.>/?".to_string()).as_enotation()
    );

    let output = ENotation::from_str("世界");
    assert_eq!(output, Identifier("世界".to_string()).as_enotation());

    let output = ENotation::from_str("本好きの下剋上");
    assert_eq!(
        output,
        Identifier("本好きの下剋上".to_string()).as_enotation()
    );
}

#[test]
fn parse_string() {
    use ENotationBody::*;
    let output = ENotation::from_str("\"abc\"");
    assert_eq!(output, Str("abc".to_string()).as_enotation())
}

#[test]
fn parse_quoting() {
    use ENotationBody::{Integer as I, List as L, QuasiQuote as QQ, Quote as Q, Syntax as S};
    let output = ENotation::from_str("'(1 2 3)");
    assert_eq!(
        output,
        Q(L(vec![
            I(1).as_enotation(),
            I(2).as_enotation(),
            I(3).as_enotation()
        ])
        .as_enotation()
        .into())
        .as_enotation()
    );

    let output = ENotation::from_str("`(1 2 3)");
    assert_eq!(
        output,
        QQ(L(vec![
            I(1).as_enotation(),
            I(2).as_enotation(),
            I(3).as_enotation()
        ])
        .as_enotation()
        .into())
        .as_enotation()
    );

    let output = ENotation::from_str("#'(1 2 3)");
    assert_eq!(
        output,
        S(L(vec![
            I(1).as_enotation(),
            I(2).as_enotation(),
            I(3).as_enotation()
        ])
        .as_enotation()
        .into())
        .as_enotation()
    );
}

#[test]
fn parse_set() {
    use ENotationBody::{Integer as I, Set as S};
    let output = ENotation::from_str("#{1 2 3}");
    assert_eq!(
        output,
        S(vec![
            I(1).as_enotation(),
            I(2).as_enotation(),
            I(3).as_enotation()
        ])
        .as_enotation()
    );

    // empty set
    let output = ENotation::from_str("#{}");
    assert_eq!(output, S(vec![]).as_enotation());
}

#[test]
fn parse_object() {
    use ENotationBody::{Integer as I, Object as O};
    let output = ENotation::from_str("{a: 2, b: 3}");
    assert_eq!(
        output,
        O(vec![
            ("a".to_string(), I(2).as_enotation()),
            ("b".to_string(), I(3).as_enotation())
        ])
        .as_enotation()
    );

    // unnamed object
    let output = ENotation::from_str("{1, 2, 3}");
    assert_eq!(
        output,
        O(vec![
            ("0".to_string(), I(1).as_enotation()),
            ("1".to_string(), I(2).as_enotation()),
            ("2".to_string(), I(3).as_enotation())
        ])
        .as_enotation()
    );

    // empty object
    let output = ENotation::from_str("{}");
    assert_eq!(output, O(vec![]).as_enotation());
}

#[test]
fn parse_comment() {
    let output = ENotationParser::parse(Rule::COMMENT, "; this is a comment")
        .unwrap()
        .peek();
    assert!(output.is_none());

    let output = ENotationParser::parse(Rule::COMMENT, "#;1").unwrap().peek();
    assert!(output.is_none());

    let output = ENotationParser::parse(Rule::COMMENT, "#;(1 2 3)")
        .unwrap()
        .peek();
    assert!(output.is_none());
}
