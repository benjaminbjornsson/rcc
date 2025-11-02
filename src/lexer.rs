#[derive(Debug)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

#[derive(Debug)]
pub enum Const {
    Int(i64),
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Constant(Const),
    Keyword(Keyword),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter,
    InvalidConstSuffix,
    InvalidIntegerLiteral(std::num::ParseIntError),
}

pub struct Lexer<'a> {
    src: &'a str,
    iter: std::str::CharIndices<'a>,
    peeked: Option<(usize, char)>,
    pos: usize,
}

impl From<std::num::ParseIntError> for LexerError {
    fn from(value: std::num::ParseIntError) -> Self {
        LexerError::InvalidIntegerLiteral(value)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            iter: src.char_indices(),
            peeked: None,
            pos: 0,
        }
    }

    fn peek_char(&mut self) -> Option<(usize, char)> {
        if self.peeked.is_none() {
            self.peeked = self.iter.next();
        }

        self.peeked
    }

    fn consume_char(&mut self) -> Option<(usize, char)> {
        let out = self.peek_char();
        self.peeked = None;
        if let Some((i, ch)) = out {
            self.pos = i + ch.len_utf8();
        }

        out
    }

    fn skip_ws(&mut self) {
        while let Some((_, ch)) = self.peek_char() {
            if ch.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let start = self.pos;
        self.consume_char();
        while let Some((_, ch)) = self.peek_char() {
            if ch == '_' || ch.is_alphanumeric() {
                self.consume_char();
            } else {
                break;
            }
        }

        let identifier = &self.src[start..self.pos];

        match identifier {
            "int" => Token::Keyword(Keyword::Int),
            "void" => Token::Keyword(Keyword::Void),
            "return" => Token::Keyword(Keyword::Return),
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    fn constant(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        self.consume_char();
        loop {
            match self.peek_char() {
                Some((_, ch)) if ch.is_ascii_digit() => {
                    self.consume_char();
                }
                _ => break,
            }
        }

        if let Some((_, ch)) = self.peek_char() {
            if ch.is_alphabetic() || ch == '_' {
                return Err(LexerError::InvalidConstSuffix);
            }
        }

        let n: i64 = self.src[start..self.pos].parse()?;
        Ok(Token::Constant(Const::Int(n)))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ws();

        match self.peek_char() {
            Some((_, ch)) if ch == '_' || ch.is_ascii_alphabetic() => Some(Ok(self.identifier())),
            Some((_, ch)) if ch.is_ascii_digit() => Some(self.constant()),
            Some((_, '(')) => {
                self.consume_char();
                Some(Ok(Token::OpenParenthesis))
            }
            Some((_, ')')) => {
                self.consume_char();
                Some(Ok(Token::CloseParenthesis))
            }
            Some((_, '{')) => {
                self.consume_char();
                Some(Ok(Token::OpenBrace))
            }
            Some((_, '}')) => {
                self.consume_char();
                Some(Ok(Token::CloseBrace))
            }
            Some((_, ';')) => {
                self.consume_char();
                Some(Ok(Token::Semicolon))
            }
            Some((_, _)) => Some(Err(LexerError::InvalidCharacter)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier() -> Result<(), LexerError> {
        let lexer = Lexer::new("foo");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(tokens.as_slice(), [Token::Identifier(s)] if s == "foo"));
        Ok(())
    }

    #[test]
    fn invalid_identifier() {
        let lexer = Lexer::new("123bar");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>();
        assert!(tokens.is_err());
    }

    #[test]
    fn constant() -> Result<(), LexerError> {
        let lexer = Lexer::new("42");
        let tokens: Vec<Token> = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(
            tokens.as_slice(),
            [Token::Constant(Const::Int(42))]
        ));
        Ok(())
    }

    #[test]
    fn open_parenthesis() -> Result<(), LexerError> {
        let lexer = Lexer::new("(");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(tokens.as_slice(), [Token::OpenParenthesis]));
        Ok(())
    }

    #[test]
    fn close_parenthesis() -> Result<(), LexerError> {
        let lexer = Lexer::new(")");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(tokens.as_slice(), [Token::CloseParenthesis]));
        Ok(())
    }

    #[test]
    fn open_brace() -> Result<(), LexerError> {
        let lexer = Lexer::new("{");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(tokens.as_slice(), [Token::OpenBrace]));
        Ok(())
    }

    #[test]
    fn close_brace() -> Result<(), LexerError> {
        let lexer = Lexer::new("}");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(tokens.as_slice(), [Token::CloseBrace]));
        Ok(())
    }

    #[test]
    fn semicolon() -> Result<(), LexerError> {
        let lexer = Lexer::new(";");
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(tokens.as_slice(), [Token::Semicolon]));
        Ok(())
    }

    #[test]
    fn simple_main() -> Result<(), LexerError> {
        let file = "int main(void) {
            return 2;
        }";
        let lexer = Lexer::new(file);
        let tokens = lexer.collect::<Result<Vec<_>, LexerError>>()?;
        assert!(matches!(
            tokens.as_slice(),
            [
                Token::Keyword(Keyword::Int),
                Token::Identifier(identifier),
                Token::OpenParenthesis,
                Token::Keyword(Keyword::Void),
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Keyword(Keyword::Return),
                Token::Constant(Const::Int(2)),
                Token::Semicolon,
                Token::CloseBrace,
            ] if identifier == "main"
        ));
        Ok(())
    }
}
