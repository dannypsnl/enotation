use std::fmt::Display;

use pest_ast::FromPest;

use crate::{literal::Identifier, ENotation, Rule};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_pair))]
pub struct ObjectPair {
    pub key: Identifier,
    pub value: ENotation,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object))]
pub struct Object {
    pub pairs: Vec<ObjectPair>,
}

impl Display for ObjectPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, pair) in self.pairs.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", pair)?;
            } else {
                write!(f, ", {}", pair)?;
            }
        }
        write!(f, "}}")
    }
}
