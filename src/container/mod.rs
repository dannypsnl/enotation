use crate::{ENotation, SetDebugFileName};

#[cfg(test)]
mod tests;

pub mod list;
use list::*;

pub mod set;
use set::*;

pub mod vector;
use vector::*;

pub mod object;
use object::*;

use std::fmt::Display;

use pest_ast::FromPest;

use crate::Rule;

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::container))]
pub enum Container {
    List(List),
    Vector(Vector),
    Set(Set),
    Object(Object),
}

impl SetDebugFileName for Container {
    fn set_debug_file_name(&mut self, file_name: &str) {
        match self {
            Container::List(l) => l.set_debug_file_name(file_name),
            Container::Vector(v) => v.set_debug_file_name(file_name),
            Container::Set(s) => s.set_debug_file_name(file_name),
            Container::Object(o) => o.set_debug_file_name(file_name),
        }
    }
}

impl Vector {
    pub fn elems(&self) -> &Vec<ENotation> {
        match self {
            Vector::PV(pvector) => &pvector.elems,
            Vector::BV(bvector) => &bvector.elems,
        }
    }
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
            Container::Vector(v) => write!(f, "{}", v),
            Container::Set(s) => write!(f, "{}", s),
            Container::Object(o) => write!(f, "{}", o),
        }
    }
}
