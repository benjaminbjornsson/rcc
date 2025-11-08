use core::fmt;

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

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Return => write!(f, "return"),
            Self::Void => write!(f, "void"),
        }
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(int) => write!(f, "{int}"),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Constant(constant) => write!(f, "{constant}"),
            Self::Keyword(keyword) => write!(f, "{keyword}"),
            Self::OpenParenthesis => write!(f, "("),
            Self::CloseParenthesis => write!(f, ")"),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
            Self::Semicolon => write!(f, ";"),
        }
    }
}
