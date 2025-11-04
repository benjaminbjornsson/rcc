use super::statement::Statement;
use super::{ParseError, Parser};
use crate::token::{Keyword, TokenKind};

pub enum Function {
    Function { name: String, body: Statement },
}

impl Function {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(TokenKind::Keyword(Keyword::Int))?;

        let token = parser.next()?;
        let identifier = match token.kind {
            TokenKind::Identifier(i) => i,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        parser.expect(TokenKind::OpenParenthesis)?;
        parser.expect(TokenKind::Keyword(Keyword::Void))?;
        parser.expect(TokenKind::CloseParenthesis)?;

        parser.expect(TokenKind::OpenBrace)?;
        let statement = Statement::parse(parser)?;
        parser.expect(TokenKind::CloseBrace)?;

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
