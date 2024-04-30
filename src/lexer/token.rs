#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
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
#[allow(unused)]
impl<'a> Token<'a> {
    pub fn from_string(string: &str) -> Self {
        match string {
            _ if string.starts_with('=') => Token {
                token_type: TokenType::ASSIGN,
                literal: "=",
            },
            _ if string.starts_with('+') => Token {
                token_type: TokenType::PLUS,
                literal: "+",
            },
            _ if string.starts_with(',') => Token {
                token_type: TokenType::COMMA,
                literal: ",",
            },
            _ if string.starts_with(';') => Token {
                token_type: TokenType::SEMICOLON,
                literal: ";",
            },
            _ if string.starts_with('(') => Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            _ if string.starts_with(')') => Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            _ if string.starts_with('{') => Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            _ if string.starts_with('}') => Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
            "0" => Token {
                token_type: TokenType::EOF,
                literal: "",
            },
            _ => Token {
                token_type: TokenType::ILLEGAL,
                literal: "",
            },
        }
    }
}
