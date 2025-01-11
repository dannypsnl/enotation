use std::fmt::Display;

use pest_ast::FromPest;

use crate::{ENotation, Rule};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::set))]
pub struct Set {
    pub elems: Vec<ENotation>,
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{{")?;
        for (i, v) in self.elems.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", v)?;
            } else {
                write!(f, " {}", v)?;
            }
        }
        write!(f, "}}")
    }
}
