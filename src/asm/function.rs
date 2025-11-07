use crate::asm::Function;
use crate::pretty::Pretty;
use std::fmt;

fn function_name(name: &str) -> String {
    if cfg!(target_os = "macos") {
        return format!("_{}", name);
    } else {
        name.to_string()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Function {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, _: usize) -> fmt::Result {
        match self {
            Function::Function { name, instructions } => {
                let name = function_name(name);
                writeln!(f, "{}.globl {}", " ".repeat(4), name)?;
                writeln!(f, "{}:", name)?;

                for instr in instructions.0.iter() {
                    instr.fmt_with(f, 1)?;
                }

                Ok(())
            }
        }
    }
}
