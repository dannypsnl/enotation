use std::fmt::Display;

use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Pest(pest::error::Error<Rule>),
}

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

fn parse_bool(input: Span) -> Result<bool, ()> {
    match input.as_str() {
        "#t" => Ok(true),
        "#f" => Ok(false),
        _ => Ok(true),
    }
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::boolean))]
pub struct Boolean {
    #[pest_ast(outer(with(parse_bool), with(Result::unwrap)))]
    pub value: bool,
}

fn parse_char(input: Span) -> Result<char, ()> {
    match input.as_str() {
        "#\\newline" => Ok('\n'),
        "#\\return" => Ok('\r'),
        "#\\space" => Ok(' '),
        "#\\tab" => Ok('\t'),
        _ => {
            let mut chars = input.as_str().chars();
            if chars.next() == Some('#') && chars.next() == Some('\\') {
                match chars.next() {
                    Some(c) => Ok(c),
                    None => Err(()),
                }
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::char))]
pub struct Char {
    #[pest_ast(outer(with(parse_char), with(Result::unwrap)))]
    pub value: char,
}

fn parse_int(input: Span) -> Result<i64, ()> {
    let mut chars = input.as_str().chars();
    let sign = match chars.next() {
        Some('+') => 1,
        Some('-') => -1,
        Some(_) => {
            chars = input.as_str().chars();
            1
        }
        None => return Err(()),
    };
    let mut value = 0;
    for c in chars {
        if c == '_' {
            continue;
        }
        value = value * 10 + c.to_digit(10).ok_or(())? as i64;
    }
    Ok(sign * value)
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::int))]
pub struct Integer {
    #[pest_ast(outer(with(parse_int), with(Result::unwrap)))]
    pub value: i64,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rational))]
pub struct Rational {
    pub numerator: Integer,
    pub denominator: Integer,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::float))]
pub struct Float {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub value: f64,
}

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}
fn parse_string(input: Span) -> Result<String, ()> {
    Ok(remove_quotes(input.as_str()))
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct String_ {
    #[pest_ast(outer(with(parse_string), with(Result::unwrap)))]
    pub value: String,
}

fn parse_identifier(input: Span) -> Result<String, ()> {
    Ok(input.as_str().to_string())
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::identifier))]
pub struct Identifier {
    #[pest_ast(outer(with(parse_identifier), with(Result::unwrap)))]
    pub name: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::literal))]
pub enum Literal {
    Boolean(Boolean),
    Char(Char),
    Float(Float),
    Rational(Rational),
    Integer(Integer),
    String_(String_),
    Identifier(Identifier),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::notation))]
pub enum ENotation {
    Literal(Literal),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::list))]
pub struct List {
    pub elems: Vec<ENotation>,
}

impl Display for ENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ENotation::*;
        match self {
            Literal(l) => write!(f, "{}", l),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Literal::*;
        match self {
            Boolean(b) => write!(f, "{}", b),
            Integer(i) => write!(f, "{}", i),
            Rational(r) => write!(f, "{}", r),
            Float(a) => write!(f, "{}", a),
            Char(c) => write!(f, "{}", c),
            String_(s) => write!(f, "\"{}\"", s),
            Identifier(x) => write!(f, "{}", x),
        }
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            write!(f, "#t")
        } else {
            write!(f, "#f")
        }
    }
}
impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            ' ' => write!(f, "#\\space"),
            '\t' => write!(f, "#\\tab"),
            '\r' => write!(f, "#\\return"),
            '\n' => write!(f, "#\\newline"),
            c => write!(f, "#\\{}", c),
        }
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.denominator, self.numerator)
    }
}
impl Display for String_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
