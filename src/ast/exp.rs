use crate::ast::Exp;
use crate::error::ParseError;
use crate::parser::Parser;
use crate::token::{Const, TokenKind};

impl Exp {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.next()?;

        match token.kind {
            TokenKind::Constant(Const::Int(num)) => Ok(Self::ConstantInt(num)),
            _ => Err(ParseError::UnexpectedToken(token, TokenKind::Constant(Const::Int(0)))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn parse_const_int() {
        let lexer = Lexer::new("5");
        let mut parser = Parser::new(lexer);
        assert!(matches!(Exp::parse(&mut parser), Ok(Exp::ConstantInt(5))));
    }
}
