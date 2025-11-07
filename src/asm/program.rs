use crate::asm::Program;
use crate::pretty::Pretty;
use std::fmt;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Program {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, _: usize) -> fmt::Result {
        match self {
            Program::Program(func) => func.fmt_with(f, 0),
        }
    }
}
