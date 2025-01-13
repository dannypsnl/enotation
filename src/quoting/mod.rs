use crate::{ENotation, Rule, SetDebugFileName};
use pest_ast::FromPest;
use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::quoting))]
pub enum Quoting {
    Quote(Quote),
    QuasiQuote(QuasiQuote),
    Unquote(Unquote),
    UnquoteSplicing(UnquoteSplicing),
}

impl SetDebugFileName for Quoting {
    fn set_debug_file_name(&mut self, file_name: &str) {
        match self {
            Quoting::Quote(quote) => quote.set_debug_file_name(file_name),
            Quoting::QuasiQuote(quasi_quote) => quasi_quote.set_debug_file_name(file_name),
            Quoting::Unquote(unquote) => unquote.set_debug_file_name(file_name),
            Quoting::UnquoteSplicing(unquote_splicing) => unquote_splicing.set_debug_file_name(file_name),
        }
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::quote))]
pub struct Quote {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for Quote {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::quasiquote))]
pub struct QuasiQuote {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for QuasiQuote {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::unquote))]
pub struct Unquote {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for Unquote {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::unquote_splicing))]
pub struct UnquoteSplicing {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for UnquoteSplicing {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
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
