use super::{ParseError, Parser};
use crate::token::{Const, Token};

pub enum Exp {
    ConstantInt(i64),
}

impl Exp {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser
            .next()
            .transpose()?
            .ok_or(ParseError::UnexpectedEof)?;

        match token {
            Token::Constant(Const::Int(num)) => Ok(Self::ConstantInt(num)),
            other => Err(ParseError::UnexpectedToken(other)),
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
