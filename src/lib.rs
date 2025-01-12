pub mod container;
pub mod literal;
pub mod quoting;
pub mod syntaxing;

use std::fmt::Display;

use container::Container;
use literal::Literal;
use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;
use quoting::Quoting;

#[cfg(test)]
mod tests;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

// #[derive(Debug)]
// pub enum ReadError {
//     Io(std::io::Error),
//     Pest(pest::error::Error<Rule>),
// }

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::syntax))]
// pub struct Syntax {
//     pub value: Box<ENotation>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::quasisyntax))]
// pub struct QuasiSyntax {
//     pub value: Box<ENotation>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::unsyntax))]
// pub struct Unsyntax {
//     pub value: Box<ENotation>,
// }

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::unsyntax_splicing))]
// pub struct UnsyntaxSplicing {
//     pub value: Box<ENotation>,
// }

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::syntaxing))]
// pub enum Syntaxing {
//     Syntax(Syntax),
//     QuasiSyntax(QuasiSyntax),
//     Unsyntax(Unsyntax),
//     UnsyntaxSplicing(UnsyntaxSplicing),
// }

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::notation))]
pub enum ENotation {
    Literal(Literal),
    Container(Container),
    Quoting(Quoting),
    // Syntaxing(Syntaxing),
}

impl Display for ENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ENotation::Literal(l) => write!(f, "{}", l),
            ENotation::Container(c) => write!(f, "{}", c),
            ENotation::Quoting(q) => write!(f, "{}", q),
            // ENotation::Syntaxing(s) => write!(f, "{}", s),
        }
    }
}

// impl Display for Syntax {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "#'{}", self.value)
//     }
// }
// impl Display for QuasiSyntax {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "#`{}", self.value)
//     }
// }
// impl Display for Unsyntax {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "#,{}", self.value)
//     }
// }
// impl Display for UnsyntaxSplicing {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "#,@{}", self.value)
//     }
// }
