use crate::error::{LexerError, LexerErrorKind};
use crate::span::Span;
use crate::token::{Const, Keyword, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a str,
    iter: std::str::CharIndices<'a>,
    peeked: Option<(usize, char)>,
    pos: usize,
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

    pub fn pos(&self) -> usize {
        self.pos
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
        let span = Span {
            start,
            end: self.pos,
        };
        match identifier {
            "int" => Token {
                kind: TokenKind::Keyword(Keyword::Int),
                span,
            },
            "void" => Token {
                kind: TokenKind::Keyword(Keyword::Void),
                span,
            },
            "return" => Token {
                kind: TokenKind::Keyword(Keyword::Return),
                span,
            },
            _ => Token {
                kind: TokenKind::Identifier(identifier.to_string()),
                span,
            },
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
                return Err(LexerError {
                    kind: LexerErrorKind::InvalidConstSuffix,
                    span: Span::single(self.pos),
                });
            }
        }

        let span = Span {
            start,
            end: self.pos,
        };
        match self.src[start..self.pos].parse::<i64>() {
            Ok(n) => Ok(Token {
                kind: TokenKind::Constant(Const::Int(n)),
                span,
            }),
            Err(_) => {
                return Err(LexerError {
                    kind: LexerErrorKind::InvalidIntegerLiteral,
                    span: Span::new(start, self.pos),
                });
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ws();

        match self.peek_char() {
            Some((_, ch)) if ch == '_' || ch.is_ascii_alphabetic() => Some(Ok(self.identifier())),
            Some((_, ch)) if ch.is_ascii_digit() => Some(self.constant()),
            Some((i, '(')) => {
                self.consume_char();
                Some(Ok(Token {
                    kind: TokenKind::OpenParenthesis,
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }))
            }
            Some((i, ')')) => {
                self.consume_char();
                Some(Ok(Token {
                    kind: TokenKind::CloseParenthesis,
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }))
            }
            Some((i, '{')) => {
                self.consume_char();
                Some(Ok(Token {
                    kind: TokenKind::OpenBrace,
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }))
            }
            Some((i, '}')) => {
                self.consume_char();
                Some(Ok(Token {
                    kind: TokenKind::CloseBrace,
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }))
            }
            Some((i, ';')) => {
                self.consume_char();
                Some(Ok(Token {
                    kind: TokenKind::Semicolon,
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }))
            }
            Some((i, c)) => Some(Err(LexerError {
                kind: LexerErrorKind::UnexpectedCharacter(c),
                span: Span::single(i),
            })),
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
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(tokens.as_slice(), [TokenKind::Identifier(s)] if s == "foo"));
        Ok(())
    }

    #[test]
    fn invalid_identifier() {
        let lexer = Lexer::new("123bar");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>();
        assert!(tokens.is_err());
    }

    #[test]
    fn constant() -> Result<(), LexerError> {
        let lexer = Lexer::new("42");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(
            tokens.as_slice(),
            [TokenKind::Constant(Const::Int(42))]
        ));
        Ok(())
    }

    #[test]
    fn open_parenthesis() -> Result<(), LexerError> {
        let lexer = Lexer::new("(");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(tokens.as_slice(), [TokenKind::OpenParenthesis]));
        Ok(())
    }

    #[test]
    fn close_parenthesis() -> Result<(), LexerError> {
        let lexer = Lexer::new(")");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(tokens.as_slice(), [TokenKind::CloseParenthesis]));
        Ok(())
    }

    #[test]
    fn open_brace() -> Result<(), LexerError> {
        let lexer = Lexer::new("{");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(tokens.as_slice(), [TokenKind::OpenBrace]));
        Ok(())
    }

    #[test]
    fn close_brace() -> Result<(), LexerError> {
        let lexer = Lexer::new("}");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(tokens.as_slice(), [TokenKind::CloseBrace]));
        Ok(())
    }

    #[test]
    fn semicolon() -> Result<(), LexerError> {
        let lexer = Lexer::new(";");
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(tokens.as_slice(), [TokenKind::Semicolon]));
        Ok(())
    }

    #[test]
    fn simple_main() -> Result<(), LexerError> {
        let file = "int main(void) {
            return 2;
        }";
        let lexer = Lexer::new(file);
        let tokens = lexer
            .map(|res| res.map(|t| t.kind))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(matches!(
            tokens.as_slice(),
            [
                TokenKind::Keyword(Keyword::Int),
                TokenKind::Identifier(identifier),
                TokenKind::OpenParenthesis,
                TokenKind::Keyword(Keyword::Void),
                TokenKind::CloseParenthesis,
                TokenKind::OpenBrace,
                TokenKind::Keyword(Keyword::Return),
                TokenKind::Constant(Const::Int(2)),
                TokenKind::Semicolon,
                TokenKind::CloseBrace,
            ] if identifier == "main"
        ));
        Ok(())
    }
}
