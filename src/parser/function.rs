use super::statement::Statement;
use super::{ParseError, Parser};
use crate::token::{Keyword, Token};

pub enum Function {
    Function { name: String, body: Statement },
}

impl Function {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Keyword(Keyword::Int))?;

        let identifier = match parser.next()? {
            Token::Identifier(i) => i,
            token => return Err(ParseError::UnexpectedToken(token)),
        };

        parser.expect(Token::OpenParenthesis)?;
        parser.expect(Token::Keyword(Keyword::Void))?;
        parser.expect(Token::CloseParenthesis)?;

        parser.expect(Token::OpenBrace)?;
        let statement = Statement::parse(parser)?;
        parser.expect(Token::CloseBrace)?;

        Ok(Self::Function {
            name: identifier,
            body: statement,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::exp::Exp;

    #[test]
    fn parse_return() {
        let file = "int main(void) {
            return 2;
        }";
        let lexer = Lexer::new(file);
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Function::parse(&mut parser),
            Ok(Function::Function {
                name: identifier,
                body: Statement::Return(Exp::ConstantInt(2))
            }) if identifier == "main"
        ));
    }
}
