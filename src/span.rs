#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn single(pos: usize) -> Self {
        Self {
            start: pos,
            end: pos + 1,
        }
    }
}
