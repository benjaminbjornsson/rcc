pub mod exp;
pub mod function;
pub mod program;
pub mod statement;

use crate::lexer::{Lexer, LexerError};
use crate::token::Token;

pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken(Token),
    Lexer(LexerError),
}

impl From<LexerError> for ParseError {
    fn from(e: LexerError) -> Self {
        ParseError::Lexer(e)
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.lexer.next().transpose()? {
            Some(t) if t == expected => Ok(()),
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => return Err(ParseError::UnexpectedEof),
        }
    }

    fn next(&mut self) -> Option<Result<Token, LexerError>> {
        self.lexer.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_semicolon() {
        let lexer = Lexer::new(";");
        let mut parser = Parser::new(lexer);
        assert!(matches!(parser.expect(Token::Semicolon), Ok(())));
    }
}
