use crate::{ENotation, Rule};
use pest_ast::FromPest;
use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::quoting))]
pub enum Quoting {
    Quote(Quote),
    QuasiQuote(QuasiQuote),
    Unquote(Unquote),
    UnquoteSplicing(UnquoteSplicing),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::quote))]
pub struct Quote {
    pub value: Box<ENotation>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::quasiquote))]
pub struct QuasiQuote {
    pub value: Box<ENotation>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unquote))]
pub struct Unquote {
    pub value: Box<ENotation>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unquote_splicing))]
pub struct UnquoteSplicing {
    pub value: Box<ENotation>,
}

impl Display for Quoting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quoting::Quote(quote) => write!(f, "{}", quote),
            Quoting::QuasiQuote(quasi_quote) => write!(f, "{}", quasi_quote),
            Quoting::Unquote(unquote) => write!(f, "{}", unquote),
            Quoting::UnquoteSplicing(unquote_splicing) => write!(f, "{}", unquote_splicing),
        }
    }
}
impl Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.value)
    }
}
impl Display for QuasiQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}", self.value)
    }
}
impl Display for Unquote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ",{}", self.value)
    }
}
impl Display for UnquoteSplicing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ",@{}", self.value)
    }
}
