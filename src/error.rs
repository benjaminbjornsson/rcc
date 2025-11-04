use std::fmt;
use crate::span::{HasSpan, Span};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub enum LexerErrorKind {
    UnexpectedCharacter(char),
    InvalidConstSuffix,
    InvalidIntegerLiteral,
}

#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub span: Span,
}

impl HasSpan for LexerError {
    fn span<'a>(&'a self) -> &'a Span {
        &self.span
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexerErrorKind::UnexpectedCharacter(found) => write!(f, "unexpected token '{found}'"),
            LexerErrorKind::InvalidConstSuffix => write!(f, "invalid const suffic"),
            LexerErrorKind::InvalidIntegerLiteral => {
                write!(f, "unable to convert integer literal value to int")
            }
        }
    }
}

pub enum ParseError {
    UnexpectedEof(Span),
    UnexpectedToken(Token, TokenKind),
    UnexpectedTrailing(Token),
    Lexer(LexerError),
}

impl HasSpan for ParseError {
    fn span<'a>(&'a self) -> &'a Span {
        match self {
            Self::UnexpectedEof(span) => span,
            Self::UnexpectedToken(token, _) => &token.span,
            Self::UnexpectedTrailing(token) => &token.span,
            Self::Lexer(e) => e.span(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ParseError::UnexpectedEof(_) => write!(f, "unexpected end of file"),
            ParseError::UnexpectedToken(token, expected) => write!(f, "expected '{:?}' but found {:?}", expected, token.kind),
            ParseError::UnexpectedTrailing(token) => write!(f, "unexpected trailing '{:?}'", token),
            ParseError::Lexer(error) => write!(f, "{}", error.to_string()),
        }
    }
}

impl From<LexerError> for ParseError {
    fn from(e: LexerError) -> Self {
        ParseError::Lexer(e)
    }
}

#[derive(Debug)]
pub enum CompilerError {
    Io(std::io::Error),
    Lexer,
    Parser,
}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        CompilerError::Io(e)
    }
}

pub fn render_diagnostic(src: &str, error: &(impl HasSpan + std::fmt::Display)) {
    let start = error.span().start.min(src.len());
    let end = error.span().end.min(src.len());

    let line_start = src[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = src[end..]
        .find('\n')
        .map(|i| end + i)
        .unwrap_or(src.len());

    let line_text = &src[line_start..line_end];

    let line_no = 1 + src[..line_start]
        .chars()
        .filter(|&c| c == '\n')
        .count();

    let col_start = src[line_start..start].chars().count().max(1);

    let expand_tabs = |s: &str| s.replace('\t', "    ");
    let expanded_line = expand_tabs(line_text);

    let prefix_expanded = expand_tabs(&src[line_start..start]);
    let underline_pad = prefix_expanded.chars().count();
    let underline_len = {
        let sel = expand_tabs(&src[start..end]);
        sel.chars().count().max(1)
    };

    let mut out = String::new();
    use std::fmt::Write;

    let _ = writeln!(out, "line {}, col {}", line_no, col_start);
    let _ = writeln!(out, "{}", expanded_line);
    let _ = writeln!(
        out,
        "{}{} {}",
        " ".repeat(underline_pad),
        "^".repeat(underline_len),
        &error.to_string()
    );

    eprint!("{}", out);
}
