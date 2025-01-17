use std::fmt::Display;

use pest_ast::FromPest;

use crate::{ENotation, Rule, SetDebugFileName};

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::paren_vector))]
pub struct PVector {
    pub elems: Vec<ENotation>,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::bracket_vector))]
pub struct BVector {
    pub elems: Vec<ENotation>,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::vector))]
pub enum Vector {
    PV(PVector),
    BV(BVector),
}

impl SetDebugFileName for PVector {
    fn set_debug_file_name(&mut self, file_name: &str) {
        for ele in self.elems.iter_mut() {
            ele.set_debug_file_name(file_name);
        }
    }
}
impl SetDebugFileName for BVector {
    fn set_debug_file_name(&mut self, file_name: &str) {
        for ele in self.elems.iter_mut() {
            ele.set_debug_file_name(file_name);
        }
    }
}
impl SetDebugFileName for Vector {
    fn set_debug_file_name(&mut self, file_name: &str) {
        match self {
            Vector::PV(pvector) => pvector.set_debug_file_name(file_name),
            Vector::BV(bvector) => bvector.set_debug_file_name(file_name),
        }
    }
}

impl Display for BVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[")?;
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

impl Display for PVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#(")?;
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

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::PV(pv) => write!(f, "{}", pv),
            Vector::BV(bv) => write!(f, "{}", bv),
        }
    }
}
