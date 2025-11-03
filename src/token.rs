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
