use crate::asm::Operand;
use crate::pretty::Pretty;
use std::fmt;

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Operand {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, _: usize) -> fmt::Result {
        match self {
            Operand::Imm(int) => write!(f, "${int}"),
            Operand::Register => write!(f, "%eax"),
        }
    }
}
