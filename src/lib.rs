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
use serde::{Deserialize, Serialize};
use syntaxing::Syntaxing;

#[cfg(test)]
mod tests;


pub trait SetDebugFileName {
    fn set_debug_file_name(&mut self, file_name: &str);
}


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

impl SetDebugFileName for ENotationBody {
    fn set_debug_file_name(&mut self, file_name: &str) {
        match self {
            ENotationBody::Container(c) => c.set_debug_file_name(file_name),
            ENotationBody::Quoting(q) => q.set_debug_file_name(file_name),
            ENotationBody::Syntaxing(s) => s.set_debug_file_name(file_name),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::notation))]
pub struct ENotation {
    #[pest_ast(outer(with(DiagnosticSpan::from_pest_span)))]
    pub span: DiagnosticSpan,
    pub body: ENotationBody,
}

impl SetDebugFileName for ENotation {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.span.file = Some(file_name.to_string());
        self.body.set_debug_file_name(file_name);
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::file))]
pub struct EFile {
    pub notations: Vec<ENotation>,
    _eoi: Eoi,
}

impl SetDebugFileName for EFile {
    fn set_debug_file_name(&mut self, file_name: &str) {
        for e in self.notations.iter_mut() {
            e.set_debug_file_name(file_name);
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct DiagnosticSpan {
    pub start_line: usize,
    pub start_col: usize,
    pub start_offset: usize,

    pub end_line: usize,
    pub end_col: usize,
    pub end_offset: usize,

    pub span: String,
    pub file: Option<String>,
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
            file: None,
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
