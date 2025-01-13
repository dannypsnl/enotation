use std::fmt::Display;

use pest_ast::FromPest;

use crate::{literal::Identifier, ENotation, Rule, SetDebugFileName};

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::object_pair))]
pub struct ObjectPair {
    pub key: Identifier,
    pub value: ENotation,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::object))]
pub struct Object {
    pub pairs: Vec<ObjectPair>,
}

impl SetDebugFileName for ObjectPair {
    fn set_debug_file_name(&mut self, file_name: &str) {
        // self.key.set_debug_file_name(file_name);
        self.value.set_debug_file_name(file_name);
    }
}

impl SetDebugFileName for Object {
    fn set_debug_file_name(&mut self, file_name: &str) {
        for pair in self.pairs.iter_mut() {
            pair.set_debug_file_name(file_name);
        }
    }
}

impl Display for ObjectPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.key, self.value)
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
