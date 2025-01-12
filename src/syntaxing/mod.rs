use crate::{ENotation, Rule};
use pest_ast::FromPest;
use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::syntax))]
pub struct Syntax {
    pub value: Box<ENotation>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::quasisyntax))]
pub struct QuasiSyntax {
    pub value: Box<ENotation>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unsyntax))]
pub struct Unsyntax {
    pub value: Box<ENotation>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unsyntax_splicing))]
pub struct UnsyntaxSplicing {
    pub value: Box<ENotation>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::syntaxing))]
pub enum Syntaxing {
    Syntax(Syntax),
    QuasiSyntax(QuasiSyntax),
    Unsyntax(Unsyntax),
    UnsyntaxSplicing(UnsyntaxSplicing),
}

impl Display for Syntaxing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Syntaxing::Syntax(syntax) => write!(f, "{}", syntax),
            Syntaxing::QuasiSyntax(quasi_syntax) => write!(f, "{}", quasi_syntax),
            Syntaxing::Unsyntax(unsyntax) => write!(f, "{}", unsyntax),
            Syntaxing::UnsyntaxSplicing(unsyntax_splicing) => write!(f, "{}", unsyntax_splicing),
        }
    }
}
impl Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#'{}", self.value)
    }
}
impl Display for QuasiSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#`{}", self.value)
    }
}
impl Display for Unsyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#,{}", self.value)
    }
}
impl Display for UnsyntaxSplicing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#,@{}", self.value)
    }
}
