use crate::ast::{Exp, Statement};
use crate::error::ParseError;
use crate::parser::Parser;
use crate::token::{Keyword, TokenKind};

impl Statement {
    pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        parser.expect(TokenKind::Keyword(Keyword::Return))?;
        let exp = Exp::parse(parser)?;
        parser.expect(TokenKind::Semicolon)?;

        Ok(Self::Return(exp))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Const, Token, TokenKind};
    use crate::span::Span;

    use super::*;

    #[test]
    fn parse_return() {
        let lexer = Lexer::new("return 2;");
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Statement::parse(&mut parser),
            Ok(Statement::Return(Exp::ConstantInt(2)))
        ));
    }

    #[test]
    fn parse_missing_semicolon() {
        let lexer = Lexer::new("return 2 2");
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Statement::parse(&mut parser),
            Err(ParseError::UnexpectedToken(
                Token {
                    kind,
                    span: _
                },
                TokenKind::Semicolon
            ))
            if kind == TokenKind::Constant(Const::Int(2))));
    }

    #[test]
    fn parse_missing_semicolon_eof() {
        let lexer = Lexer::new("return 2");
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Statement::parse(&mut parser),
            Err(ParseError::UnexpectedEof(Span { start, end }))
            if start == 8 && end == 9
        ));
    }
}
