use crate::{container::set::Set, ENotation, Rule, SetDebugFileName};
use pest_ast::FromPest;
use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::syntax))]
pub struct Syntax {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for Syntax {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::quasisyntax))]
pub struct QuasiSyntax {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for QuasiSyntax {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::unsyntax))]
pub struct Unsyntax {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for Unsyntax {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::unsyntax_splicing))]
pub struct UnsyntaxSplicing {
    pub value: Box<ENotation>,
}

impl SetDebugFileName for UnsyntaxSplicing {
    fn set_debug_file_name(&mut self, file_name: &str) {
        self.value.set_debug_file_name(file_name);
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::syntaxing))]
pub enum Syntaxing {
    Syntax(Syntax),
    QuasiSyntax(QuasiSyntax),
    Unsyntax(Unsyntax),
    UnsyntaxSplicing(UnsyntaxSplicing),
}

impl SetDebugFileName for Syntaxing {
    fn set_debug_file_name(&mut self, file_name: &str) {
        match self {
            Syntaxing::Syntax(syntax) => syntax.set_debug_file_name(file_name),
            Syntaxing::QuasiSyntax(quasi_syntax) => quasi_syntax.set_debug_file_name(file_name),
            Syntaxing::Unsyntax(unsyntax) => unsyntax.set_debug_file_name(file_name),
            Syntaxing::UnsyntaxSplicing(unsyntax_splicing) => unsyntax_splicing.set_debug_file_name(file_name),
        }
    }
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
