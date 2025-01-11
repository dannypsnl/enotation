#![feature(iter_next_chunk)]
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::{fmt::Display, fs::File, io::Read, path::Path, rc::Rc};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Pest(pest::error::Error<Rule>),
}
impl From<pest::error::Error<Rule>> for ReadError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        ReadError::Pest(err)
    }
}
impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self {
        ReadError::Io(err)
    }
}

pub fn parse_path(path: &Path) -> Result<Vec<ENotation>, ReadError> {
    parse_file(File::open(path)?)
}
pub fn parse_file(mut file: File) -> Result<Vec<ENotation>, ReadError> {
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    parse_str(string.as_str())
}
pub fn parse_str(input: &str) -> Result<Vec<ENotation>, ReadError> {
    let pairs = ENotationParser::parse(Rule::file, input)?;
    Ok(pairs
        .peek()
        .unwrap()
        .into_inner()
        .filter(|p| match p.as_rule() {
            Rule::COMMENT
            | Rule::WHITESPACE
            | Rule::EOI
            | Rule::single_line_comment
            | Rule::single_notation_comment => false,
            _ => true,
        })
        .map(|p| ENotation::from_pair(p))
        .collect::<Vec<_>>())
}

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
    // {1, 2, 3}
    UnamedObject(Vec<ENotation>),
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

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
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

                Rule::unamed_object => {
                    UnamedObject(pair.into_inner().map(|p| ENotation::from_pair(p)).collect())
                }
                Rule::object => Object(
                    pair.into_inner()
                        .map(|p| {
                            let mut inner_rules = p.into_inner();
                            let key = inner_rules.next().unwrap();
                            let val = inner_rules.next().unwrap();
                            (key.as_str().to_string(), ENotation::from_pair(val))
                        })
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

impl Display for ENotationBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ENotationBody::Boolean(true) => write!(f, "#t"),
            ENotationBody::Boolean(false) => write!(f, "#f"),
            ENotationBody::Integer(i) => write!(f, "{}", i),
            ENotationBody::Rational(p, q) => write!(f, "{}/{}", p, q),
            ENotationBody::Float(a) => write!(f, "{}", a),
            ENotationBody::Char('\n') => write!(f, "#\\newline"),
            ENotationBody::Char('\r') => write!(f, "#\\return"),
            ENotationBody::Char(' ') => write!(f, "#\\space"),
            ENotationBody::Char('\t') => write!(f, "#\\tab"),
            ENotationBody::Char(c) => write!(f, "#\\{}", c),
            ENotationBody::Str(s) => write!(f, "\"{}\"", s),
            ENotationBody::Identifier(x) => write!(f, "{}", x),
            ENotationBody::List(vec) => {
                write!(f, "(")?;
                for (i, x) in vec.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", x)?;
                    } else {
                        write!(f, " {}", x)?;
                    }
                }
                write!(f, ")")
            }
            ENotationBody::Set(vec) => {
                write!(f, "#{{")?;
                for (i, x) in vec.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", x)?;
                    } else {
                        write!(f, " {}", x)?;
                    }
                }
                write!(f, "}}")
            }

            ENotationBody::UnamedObject(vec) => {
                write!(f, "{{")?;
                for (i, x) in vec.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", x)?;
                    } else {
                        write!(f, ", {}", x)?;
                    }
                }
                write!(f, "}}")
            }
            ENotationBody::Object(vec) => {
                write!(f, "{{")?;
                for (i, (key, value)) in vec.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}: {}", key, value)?;
                    } else {
                        write!(f, ", {}: {}", key, value)?;
                    }
                }
                write!(f, "}}")
            }
            ENotationBody::Quote(enotation) => write!(f, "'{}", enotation),
            ENotationBody::QuasiQuote(enotation) => {
                write!(f, "`{}", enotation)
            }
            ENotationBody::Unquote(enotation) => {
                write!(f, ",{}", enotation)
            }
            ENotationBody::UnquoteSplicing(enotation) => {
                write!(f, ",@{}", enotation)
            }
            ENotationBody::Syntax(enotation) => {
                write!(f, "#'{}", enotation)
            }
            ENotationBody::QuasiSyntax(enotation) => {
                write!(f, "#`{}", enotation)
            }
            ENotationBody::Unsyntax(enotation) => {
                write!(f, "#,{}", enotation)
            }
            ENotationBody::UnsyntaxSplicing(enotation) => {
                write!(f, "#,@{}", enotation)
            }
        }
    }
}

impl PartialEq for ENotation {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}

impl Display for ENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.body.fmt(f)
    }
}
