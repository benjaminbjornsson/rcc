use crate::ast::Function;
use crate::pretty::{self, Pretty};
use std::fmt;

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Function {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        match self {
            Self::Function { name, body } => {
                writeln!(f, "Function(")?;
                pretty::indent(f, depth + 1)?;
                writeln!(f, "name={},", name)?;
                pretty::indent(f, depth + 1)?;
                write!(f, "body=")?;
                body.fmt_with(f, depth + 1)?;
                pretty::indent(f, depth)?;
                writeln!(f, ")")
            }
        }
    }
}
