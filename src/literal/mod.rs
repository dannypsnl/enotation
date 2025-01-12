use std::fmt::Display;

use pest::Span;
use pest_ast::FromPest;

use crate::Rule;

#[cfg(test)]
mod test;

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
    println!("hello {:?}", input);
    match input.as_str() {
        "#\\newline" => Ok('\n'),
        "#\\return" => Ok('\r'),
        "#\\space" => Ok(' '),
        "#\\tab" => Ok('\t'),
        i => {
            let mut chars = i.chars();
            if chars.next() == Some('#') && chars.next() == Some('\\') {
                if let Some(c) = chars.next() {
                    return Ok(c);
                }
            }
            Err(())
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

fn parse_rational(input: Span) -> Result<(i64, i64), ()> {
    let v = input.as_str().split('/').collect::<Vec<_>>();
    let p: i64 = v[0].parse().map_err(|_| ())?;
    let q: i64 = v[1].parse().map_err(|_| ())?;
    Ok((p, q))
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rational))]
pub struct Rational {
    #[pest_ast(outer(with(parse_rational), with(Result::unwrap)))]
    pub value: (i64, i64),
}

fn span_into_str(span: Span) -> &str {
    span.as_str()
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
    Int(Integer),
    String_(String_),
    Identifier(Identifier),
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
        write!(f, "{}/{}", self.value.0, self.value.1)
    }
}
impl Display for String_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Char(c) => write!(f, "{}", c),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::Rational(r) => write!(f, "{}", r),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::String_(s) => write!(f, "{}", s),
            Literal::Identifier(i) => write!(f, "{}", i),
        }
    }
}
