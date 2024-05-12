#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'lexing> {
    Illegal(&'lexing str),
    Eof,

    Ident(&'lexing str),

    Int(&'lexing str),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Eq,
    NotEq,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
