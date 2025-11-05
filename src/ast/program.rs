use crate::ast::{Function, Program};
use crate::error::ParseError;
use crate::parser::Parser;

impl Program {
    pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let function_definition = Function::parse(parser)?;

        Ok(Self::FunctionDefinition(function_definition))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenKind};
    use crate::ast::{Exp, Statement};

    #[test]
    fn return_int() {
        let file = "int main(void) {
            return 2;
        }";
        let lexer = Lexer::new(file);
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Program::parse(&mut parser),
            Ok(Program::FunctionDefinition(Function::Function {
                name: identifier,
                body: Statement::Return(Exp::ConstantInt(2))
            })) if identifier == "main"
        ));
    }

    #[test]
    fn return_int_missing_semicolon() {
        let file = "int main(void) {
            return 2
        }";
        let lexer = Lexer::new(file);
        let mut parser = Parser::new(lexer);
        assert!(matches!(
            Program::parse(&mut parser),
            Err(ParseError::UnexpectedToken(Token { kind, span: _ }, TokenKind::Semicolon ))
            if kind == TokenKind::CloseBrace
        ));
    }
}
