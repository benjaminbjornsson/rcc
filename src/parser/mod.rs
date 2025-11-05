use crate::ast::{Exp, Function, Program, Statement};
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::span::Span;
use crate::token::{Const, Keyword, Token, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.parse_program()
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

impl<'a> Parser<'a> {
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let function_definition = self.parse_function()?;

        self.expect_eof()?;

        Ok(Program::FunctionDefinition(function_definition))
    }

    pub fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Int))?;

        let token = self.next()?;
        let identifier = match token.kind {
            TokenKind::Identifier(i) => i,
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token,
                    TokenKind::Identifier(String::from("")),
                ))
            }
        };

        self.expect(TokenKind::OpenParenthesis)?;
        self.expect(TokenKind::Keyword(Keyword::Void))?;
        self.expect(TokenKind::CloseParenthesis)?;

        self.expect(TokenKind::OpenBrace)?;
        let statement = self.parse_statement()?;
        self.expect(TokenKind::CloseBrace)?;

        Ok(Function::Function {
            name: identifier,
            body: statement,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Return))?;
        let exp = self.parse_exp()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Statement::Return(exp))
    }

    fn parse_exp(&mut self) -> Result<Exp, ParseError> {
        let token = self.next()?;

        match token.kind {
            TokenKind::Constant(Const::Int(num)) => Ok(Exp::ConstantInt(num)),
            _ => Err(ParseError::UnexpectedToken(
                token,
                TokenKind::Constant(Const::Int(0)),
            )),
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

    mod program {
        use super::*;
        use crate::ast::{Exp, Statement};
        use crate::lexer::Lexer;
        use crate::token::{Token, TokenKind};

        #[test]
        fn return_int() {
            let file = "int main(void) {
                return 2;
            }";
            let lexer = Lexer::new(file);
            let mut parser = Parser::new(lexer);
            assert!(matches!(
                parser.parse_program(),
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
                parser.parse_program(),
                Err(ParseError::UnexpectedToken(Token { kind, span: _ }, TokenKind::Semicolon ))
                if kind == TokenKind::CloseBrace
            ));
        }
    }

    mod function {
        use super::*;
        use crate::ast::Exp;
        use crate::lexer::Lexer;

        #[test]
        fn parse_return() {
            let file = "int main(void) {
                return 2;
            }";
            let lexer = Lexer::new(file);
            let mut parser = Parser::new(lexer);
            assert!(matches!(
                parser.parse_function(),
                Ok(Function::Function {
                    name: identifier,
                    body: Statement::Return(Exp::ConstantInt(2))
                }) if identifier == "main"
            ));
        }
    }

    mod statement {
        use super::*;
        use crate::lexer::Lexer;
        use crate::span::Span;
        use crate::token::{Const, Token, TokenKind};

        #[test]
        fn parse_return() {
            let lexer = Lexer::new("return 2;");
            let mut parser = Parser::new(lexer);
            assert!(matches!(
                parser.parse_statement(),
                Ok(Statement::Return(Exp::ConstantInt(2)))
            ));
        }

        #[test]
        fn parse_missing_semicolon() {
            let lexer = Lexer::new("return 2 2");
            let mut parser = Parser::new(lexer);
            assert!(matches!(
                parser.parse_statement(),
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
                parser.parse_statement(),
                Err(ParseError::UnexpectedEof(Span { start, end }))
                if start == 8 && end == 9
            ));
        }
    }

    mod exp {
        use super::*;

        #[test]
        fn parse_const_int() {
            let lexer = Lexer::new("5");
            let mut parser = Parser::new(lexer);
            assert!(matches!(parser.parse_exp(), Ok(Exp::ConstantInt(5))));
        }
    }
}
