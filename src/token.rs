use crate::span::Span;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Const {
    Int(i64),
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Constant(Const),
    Keyword(Keyword),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
