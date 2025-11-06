use std::fmt;

const INDENT: usize = 4;

pub fn indent(f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
    f.write_str(&" ".repeat(INDENT * depth))
}

pub trait Pretty {
    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
}
