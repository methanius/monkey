#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'lexing> {
    ILLEGAL(&'lexing str),
    EOF,
    IDENT(&'lexing str),
    INT(&'lexing str),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}
