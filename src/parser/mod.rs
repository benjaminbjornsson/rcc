use crate::lexer::Lexer;
use crate::span::Span;
use crate::token::{Token, TokenKind};
use crate::error::ParseError;
use crate::ast::Program;

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

    pub fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        match self.next()? {
            token if token.kind == expected => Ok(()),
            token => Err(ParseError::UnexpectedToken(token, expected)),
        }
    }

    pub fn expect_eof(&mut self) -> Result<(), ParseError> {
        match self.lexer.next().transpose()? {
            None => Ok(()),
            Some(t) => Err(ParseError::UnexpectedTrailing(t)),
        }
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        match self.lexer.next() {
            None => Err(ParseError::UnexpectedEof(Span::single(self.lexer.pos()))),
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
