pub mod exp;
pub mod function;
pub mod program;
pub mod statement;

use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::parser::program::Program;
use crate::error::ParseError;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let program = Program::parse(self)?;

        self.expect_eof()?;

        Ok(program)
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        match self.next()? {
            token if token.kind == expected => Ok(()),
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn expect_eof(&mut self) -> Result<(), ParseError> {
        match self.lexer.next().transpose()? {
            None => Ok(()),
            Some(t) => Err(ParseError::UnexpectedTrailing(t)),
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        match self.lexer.next() {
            None => Err(ParseError::UnexpectedEof),
            Some(Ok(t)) => Ok(t),
            Some(Err(e)) => Err(ParseError::Lexer(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_semicolon() {
        let lexer = Lexer::new(";");
        let mut parser = Parser::new(lexer);
        assert!(matches!(parser.expect(TokenKind::Semicolon), Ok(())));
    }
}
