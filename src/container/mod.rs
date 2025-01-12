use crate::ENotation;

#[cfg(test)]
mod tests;

mod list;
use list::*;

mod set;
use set::*;

mod unamed_object;
use unamed_object::*;

mod object;
use object::*;

use std::fmt::Display;

use pest_ast::FromPest;

use crate::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::container))]
pub enum Container {
    List(List),
    Set(Set),
    UnamedObject(UnamedObject),
    Object(Object),
}

impl List {
    pub fn elems(&self) -> &Vec<ENotation> {
        match self {
            List::PL(plist) => &plist.elems,
            List::BL(blist) => &blist.elems,
        }
    }
}

impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Container::List(l) => write!(f, "{}", l),
            Container::Set(s) => write!(f, "{}", s),
            Container::UnamedObject(uo) => write!(f, "{}", uo),
            Container::Object(o) => write!(f, "{}", o),
        }
    }
}
