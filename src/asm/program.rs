use crate::asm::Program;
use crate::pretty::Pretty;
use std::fmt;

fn prolog(_f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[cfg(target_os = "linux")]
    {
        writeln!(_f)?;
        writeln!(_f, r#"{}.section .note.GNU-stack,"",@progbits"#, " ".repeat(4))?;
    }

    Ok(())
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, 0)
    }
}

impl Pretty for Program {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, _: usize) -> fmt::Result {
        match self {
            Program::Program(func) => func.fmt_with(f, 0)?,
        }

        prolog(f)
    }
}
