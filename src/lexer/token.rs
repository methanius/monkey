#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
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
