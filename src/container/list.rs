use std::fmt::Display;

use pest_ast::FromPest;

use crate::{ENotation, Rule};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::paren_list))]
pub struct PList {
    pub elems: Vec<ENotation>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::bracket_list))]
pub struct BList {
    pub elems: Vec<ENotation>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::list))]
pub enum List {
    PL(PList),
    BL(BList),
}

impl List {
    pub fn elems(&self) -> &Vec<ENotation> {
        match self {
            List::PL(plist) => &plist.elems,
            List::BL(blist) => &blist.elems,
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
