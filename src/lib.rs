pub mod container;
pub mod literal;

use std::fmt::Display;

use container::Container;
use literal::Literal;
use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

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
// #[pest_ast(rule(Rule::set))]
// pub struct Set {
//     pub elems: Vec<ENotation>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::unamed_object))]
// pub struct UnamedObject {
//     pub elems: Vec<ENotation>,
// }

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::object_pair))]
// pub struct ObjectPair {
//     #[pest_ast(outer(with(parse_identifier), with(Result::unwrap)))]
//     pub key: String,
//     pub value: ENotation,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::object))]
// pub struct Object {
//     pub elems: Vec<ObjectPair>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::container))]
// pub enum Container {
//     List(List),
//     Set(Set),
//     UnamedObject(UnamedObject),
//     Object(Object),
// }

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::quote))]
// pub struct Quote {
//     pub value: Box<ENotation>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::quasiquote))]
// pub struct QuasiQuote {
//     pub value: Box<ENotation>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::unquote))]
// pub struct Unquote {
//     pub value: Box<ENotation>,
// }
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::unquote_splicing))]
// pub struct UnquoteSplicing {
//     pub value: Box<ENotation>,
// }

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::quoting))]
// pub enum Quoting {
//     Quote(Quote),
//     QuasiQuote(QuasiQuote),
//     Unquote(Unquote),
//     UnquoteSplicing(UnquoteSplicing),
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
    // Quoting(Quoting),
    // Syntaxing(Syntaxing),
}

impl Display for ENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ENotation::Literal(l) => write!(f, "{}", l),
            ENotation::Container(c) => write!(f, "{}", c),
            // ENotation::Quoting(q) => write!(f, "{}", q),
            // ENotation::Syntaxing(s) => write!(f, "{}", s),
        }
    }
}

// impl Display for ENotation {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#?}", self)
//     }
// }

// impl Display for Container {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#?}", self)
//     }
// }

// impl Display for Set {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "#{{")?;
//         for (i, v) in self.elems.iter().enumerate() {
//             if i == 0 {
//                 write!(f, "{}", v)?;
//             } else {
//                 write!(f, " {}", v)?;
//             }
//         }
//         write!(f, "}}")
//     }
// }
// impl Display for UnamedObject {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{")?;
//         for (i, v) in self.elems.iter().enumerate() {
//             if i == 0 {
//                 write!(f, "{}", v)?;
//             } else {
//                 write!(f, ", {}", v)?;
//             }
//         }
//         write!(f, "}}")
//     }
// }
// impl Display for Object {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{")?;
//         for (i, v) in self.elems.iter().enumerate() {
//             if i == 0 {
//                 write!(f, "{}: {}", v.key, v.value)?;
//             } else {
//                 write!(f, ", {}: {}", v.key, v.value)?;
//             }
//         }
//         write!(f, "}}")
//     }
// }
// impl Display for BList {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(")?;
//         for (i, v) in self.elems.iter().enumerate() {
//             if i == 0 {
//                 write!(f, "{}", v)?;
//             } else {
//                 write!(f, " {}", v)?;
//             }
//         }
//         write!(f, ")")
//     }
// }

// impl Display for PList {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[")?;
//         for (i, v) in self.elems.iter().enumerate() {
//             if i == 0 {
//                 write!(f, "{}", v)?;
//             } else {
//                 write!(f, " {}", v)?;
//             }
//         }
//         write!(f, "]")
//     }
// }

// impl Display for List {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             List::PL(pl) => write!(f, "{}", pl),
//             List::BL(bl) => write!(f, "{}", bl),
//         }
//     }
// }

// impl Display for Quote {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "'{}", self.value)
//     }
// }
// impl Display for QuasiQuote {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "`{}", self.value)
//     }
// }
// impl Display for Unquote {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, ",{}", self.value)
//     }
// }
// impl Display for UnquoteSplicing {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, ",@{}", self.value)
//     }
// }
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
