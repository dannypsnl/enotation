#![feature(iter_next_chunk)]
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::rc::Rc;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticSpan {
    pub start_line: usize,
    pub start_col: usize,
    pub start_offset: usize,

    pub end_line: usize,
    pub end_col: usize,
    pub end_offset: usize,

    pub span: String,
}

impl Default for DiagnosticSpan {
    fn default() -> Self {
        Self {
            start_line: 0,
            start_col: 0,
            start_offset: 0,
            end_line: 0,
            end_col: 0,
            end_offset: 0,
            span: String::new(),
        }
    }
}

impl DiagnosticSpan {
    pub fn from_pest_span(span: &pest::Span<'_>) -> Self {
        let (start_line, start_col) = span.start_pos().line_col();
        let (end_line, end_col) = span.end_pos().line_col();
        let start_offset = span.start();
        let end_offset = span.end();
        let source = span.as_str().to_string();
        Self {
            start_line,
            start_col,
            start_offset,
            end_line,
            end_col,
            end_offset,
            span: source,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ENotationBody {
    Boolean(bool),
    Integer(i64),
    Rational(i64, i64),
    // .123 0.13 +.3 -.1
    Float(f64),
    // #\c #\newline #\return #\space #\tab
    Char(char),
    Str(String),
    Identifier(String),

    // (a b c)
    List(Vec<ENotation>),
    // #{a b c}
    Set(Vec<ENotation>),
    // {a: 1, b: 2}
    Object(Vec<(String, ENotation)>),
    // '(1 c a)
    Quote(Rc<ENotation>),
    // `(1 c a)
    QuasiQuote(Rc<ENotation>),
    // ,a
    Unquote(Rc<ENotation>),
    // ,@(a b c)
    UnquoteSplicing(Rc<ENotation>),
    // #'(1 c a)
    Syntax(Rc<ENotation>),
    // #`(1 c a)
    QuasiSyntax(Rc<ENotation>),
    // #,1
    Unsyntax(Rc<ENotation>),
    // #,@(a b c)
    UnsyntaxSplicing(Rc<ENotation>),
}

#[derive(Debug, Clone)]
pub struct ENotation {
    pub span: DiagnosticSpan,
    pub body: ENotationBody,
}

impl PartialEq for ENotation {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}

fn extract_object_pair((index, pair): (usize, Pair<Rule>)) -> (String, ENotation) {
    match pair.as_rule() {
        Rule::object_pair => {
            let mut inner_rules = pair.into_inner();
            let key = inner_rules.next().unwrap();
            let val = inner_rules.next().unwrap();
            (key.as_str().to_string(), ENotation::from_pair(val))
        }
        _ => (index.to_string(), ENotation::from_pair(pair)),
    }
}

impl ENotation {
    fn from_pair(pair: Pair<Rule>) -> Self {
        use ENotationBody::*;
        ENotation {
            span: DiagnosticSpan::from_pest_span(&pair.as_span()),
            body: match pair.as_rule() {
                Rule::boolean_true => Boolean(true),
                Rule::boolean_false => Boolean(false),

                Rule::regular_char => Char(pair.as_str().chars().nth(2).unwrap()),
                Rule::char_newline => Char('\n'),
                Rule::char_return => Char('\r'),
                Rule::char_space => Char(' '),
                Rule::char_tab => Char('\t'),

                Rule::int => Integer(pair.as_str().parse().unwrap()),
                Rule::rational => {
                    let mut inner_rules = pair.into_inner();
                    let p = inner_rules.next().unwrap().as_str().parse().unwrap();
                    let q = inner_rules.next().unwrap().as_str().parse().unwrap();
                    Rational(p, q)
                }
                Rule::float => Float(pair.as_str().parse().unwrap()),
                Rule::string => Str(remove_quotes(pair.as_str())),
                Rule::identifier => Identifier(pair.as_str().to_string()),

                Rule::list => List(pair.into_inner().map(ENotation::from_pair).collect()),
                Rule::set => Set(pair.into_inner().map(ENotation::from_pair).collect()),
                Rule::object => Object(
                    pair.into_inner()
                        .enumerate()
                        .map(extract_object_pair)
                        .collect(),
                ),

                Rule::quote => Quote(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::quasiquote => QuasiQuote(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::unquote => Unquote(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::unquote_splicing => UnquoteSplicing(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::syntax => Syntax(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::quasisyntax => QuasiSyntax(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::unsyntax => Unsyntax(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),
                Rule::unsyntax_splicing => UnsyntaxSplicing(Rc::new(ENotation::from_pair(
                    pair.into_inner().peek().unwrap(),
                ))),

                Rule::COMMENT
                | Rule::WHITESPACE
                | Rule::SCHEME_ALPHA
                | Rule::SIGN
                | Rule::EOI
                | Rule::file
                | Rule::single_line_comment
                | Rule::single_notation_comment
                | Rule::dec_int
                | Rule::char
                | Rule::boolean
                | Rule::paren_list
                | Rule::bracket_list
                | Rule::notation
                | Rule::object_pair => {
                    unreachable!()
                }
            },
        }
    }

    pub fn from_str(input: &str) -> Self {
        let output = ENotationParser::parse(Rule::notation, input)
            .unwrap()
            .next()
            .unwrap();
        Self::from_pair(output)
    }
}
