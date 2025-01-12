pub mod container;
pub mod literal;
pub mod quoting;
pub mod syntaxing;

use std::fmt::Display;

use container::Container;
use literal::Literal;
use pest_ast::FromPest;
use pest_derive::Parser;
use quoting::Quoting;
use syntaxing::Syntaxing;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::notation_))]
pub enum ENotationBody {
    Literal(Literal),
    Container(Container),
    Quoting(Quoting),
    Syntaxing(Syntaxing),
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::notation))]
pub struct ENotation {
    #[pest_ast(outer(with(DiagnosticSpan::from_pest_span)))]
    pub span: DiagnosticSpan,
    pub body: ENotationBody,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::file))]
pub struct EFile {
    pub notations: Vec<ENotation>,
    _eoi: EOI,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI {}

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
    pub fn from_pest_span(span: pest::Span<'_>) -> Self {
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

impl Display for EFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.notations {
            writeln!(f, "{}", e)?;
        }
        Ok(())
    }
}
impl Display for ENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ENotationBody::*;
        match &self.body {
            Literal(l) => write!(f, "{}", l),
            Container(c) => write!(f, "{}", c),
            Quoting(q) => write!(f, "{}", q),
            Syntaxing(s) => write!(f, "{}", s),
        }
    }
}
