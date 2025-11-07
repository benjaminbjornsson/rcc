use crate::ast::Statement;
use crate::pretty::{self, Pretty};
use std::fmt;

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Statement {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        match self {
            Self::Return(exp) => {
                writeln!(f, "Return(")?;
                pretty::indent(f, depth + 1)?;
                exp.fmt_with(f, depth + 1)?;
                pretty::indent(f, depth)?;
                writeln!(f, ")")
            }
        }
    }
}
