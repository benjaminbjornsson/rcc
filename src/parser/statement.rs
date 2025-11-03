use super::exp::Exp;
use super::{ParseError, Parser};
use crate::token::{Keyword, Token};

pub enum Statement {
    Return(Exp),
}

impl Statement {
    pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        parser.expect(Token::Keyword(Keyword::Return))?;
        let exp = Exp::parse(parser)?;
        parser.expect(Token::Semicolon)?;

        Ok(Self::Return(exp))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Const;

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
            Err(ParseError::UnexpectedToken(Token::Constant(Const::Int(2))))
        ));
    }

    #[test]
    fn parse_missing_semicolon_eof() {
        let lexer = Lexer::new("return 2");
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Statement::parse(&mut parser),
            Err(ParseError::UnexpectedEof)
        ));
    }
}
