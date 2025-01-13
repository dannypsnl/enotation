use std::fmt::Display;

use pest_ast::FromPest;

use crate::{ENotation, Rule, SetDebugFileName};

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::paren_list))]
pub struct PList {
    pub elems: Vec<ENotation>,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::bracket_list))]
pub struct BList {
    pub elems: Vec<ENotation>,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::list))]
pub enum List {
    PL(PList),
    BL(BList),
}

impl SetDebugFileName for PList {
    fn set_debug_file_name(&mut self, file_name: &str) {
        for elem in self.elems.iter_mut() {
            elem.set_debug_file_name(file_name);
        }
    }
}

impl SetDebugFileName for BList {
    fn set_debug_file_name(&mut self, file_name: &str) {
        for elem in self.elems.iter_mut() {
            elem.set_debug_file_name(file_name);
        }
    }
}

impl SetDebugFileName for List {
    fn set_debug_file_name(&mut self, file_name: &str) {
        match self {
            List::PL(pl) => pl.set_debug_file_name(file_name),
            List::BL(bl) => bl.set_debug_file_name(file_name),
        }
    }
}

impl Display for BList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.elems.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", v)?;
            } else {
                write!(f, " {}", v)?;
            }
        }
        write!(f, "]")
    }
}

impl Display for PList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, v) in self.elems.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", v)?;
            } else {
                write!(f, " {}", v)?;
            }
        }
        write!(f, ")")
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::PL(pl) => write!(f, "{}", pl),
            List::BL(bl) => write!(f, "{}", bl),
        }
    }
}
