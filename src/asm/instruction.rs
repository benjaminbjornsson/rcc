use crate::asm::Instr;
use crate::pretty::{self, Pretty};
use std::fmt;

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Instr {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        pretty::indent(f, depth)?;

        match self {
            Instr::Mov { src, dst } => writeln!(f, "movl{}{}, {}", " ".repeat(2), dst, src),
            Instr::Ret => writeln!(f, "ret"),
        }
    }
}
