use crate::ast::{self, Exp};
use crate::pretty::Pretty;
use std::fmt;

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for ast::Exp {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, _: usize) -> fmt::Result {
        match self {
            Self::ConstantInt(n) => writeln!(f, "ConstantInt({n})"),
        }
    }
}
