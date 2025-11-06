use crate::ast::{pretty::*, Program};
use std::fmt;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Program {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        match self {
            Self::FunctionDefinition(func) => {
                writeln!(f, "Program(")?;
                indent(f, depth + 1)?;
                func.fmt_with(f, depth + 1)?;
                indent(f, depth)?;
                writeln!(f, ")")
            }
        }
    }
}
