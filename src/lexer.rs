mod token;

use token::{Token, TokenType};
pub struct Lexer<'lexing> {
    input: &'lexing str,
}

impl<'lexing> Lexer<'lexing> {
    pub fn new(code: &'lexing str) -> Self {
        Lexer { input: code }
    }

    pub fn next_token(self) -> (Token<'lexing>, Lexer<'lexing>) {
        let token = match self.input.chars().next().unwrap() {
            '=' => Token {
                token_type: TokenType::ASSIGN,
                literal: "=",
            },
            '+' => Token {
                token_type: TokenType::PLUS,
                literal: "+",
            },
            ',' => Token {
                token_type: TokenType::COMMA,
                literal: ",",
            },
            ';' => Token {
                token_type: TokenType::SEMICOLON,
                literal: ";",
            },
            '(' => Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            ')' => Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            '{' => Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            '}' => Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
            '0' => Token {
                token_type: TokenType::EOF,
                literal: "",
            },
            _ => Token {
                token_type: TokenType::ILLEGAL,
                literal: "",
            },
        };
        (token, Lexer::new(&self.input[1..]))
    }
}

pub fn lex(code: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(code);
    let mut token;
    let mut token_vec: Vec<Token> = vec![];
    loop {
        (token, lexer) = lexer.next_token();
        token_vec.push(token);
        if token_vec.last().unwrap().token_type == TokenType::EOF
            || token_vec.last().unwrap().token_type == TokenType::ILLEGAL
        {
            return token_vec;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, TokenType};
    #[test]
    fn test_next_single_literal_token() {
        const INPUT: &str = "=+(){},;";
        let token_vec = lex(INPUT);
        assert_eq!(
            token_vec,
            vec![
                Token {
                    token_type: TokenType::ASSIGN,
                    literal: "="
                },
                Token {
                    token_type: TokenType::PLUS,
                    literal: "+"
                },
                Token {
                    token_type: TokenType::LPAREN,
                    literal: "("
                },
                Token {
                    token_type: TokenType::RPAREN,
                    literal: ")"
                },
                Token {
                    token_type: TokenType::LBRACE,
                    literal: "{"
                },
                Token {
                    token_type: TokenType::RBRACE,
                    literal: "}"
                },
                Token {
                    token_type: TokenType::COMMA,
                    literal: ","
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::EOF,
                    literal: ""
                },
            ]
        );
    }

    #[test]
    fn test_next_multi_literal_token() {
        const INPUT: &str = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);";
        let token_vec = lex(INPUT);
        assert_eq!(
            token_vec,
            vec![
                Token {
                    token_type: TokenType::LET,
                    literal: "let"
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "five"
                },
                Token {
                    token_type: TokenType::ASSIGN,
                    literal: "="
                },
                Token {
                    token_type: TokenType::INT,
                    literal: "5"
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::LET,
                    literal: "let"
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "ten"
                },
                Token {
                    token_type: TokenType::ASSIGN,
                    literal: "="
                },
                Token {
                    token_type: TokenType::INT,
                    literal: "10"
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::LET,
                    literal: "let"
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "add"
                },
                Token {
                    token_type: TokenType::ASSIGN,
                    literal: "="
                },
                Token {
                    token_type: TokenType::FUNCTION,
                    literal: "fn"
                },
                Token {
                    token_type: TokenType::LPAREN,
                    literal: "("
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "x"
                },
                Token {
                    token_type: TokenType::COMMA,
                    literal: ","
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "y"
                },
                Token {
                    token_type: TokenType::RPAREN,
                    literal: ")"
                },
                Token {
                    token_type: TokenType::LBRACE,
                    literal: "{"
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "x"
                },
                Token {
                    token_type: TokenType::PLUS,
                    literal: "+"
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "y"
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::RBRACE,
                    literal: "}"
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::LET,
                    literal: "let"
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "result"
                },
                Token {
                    token_type: TokenType::ASSIGN,
                    literal: "="
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "add"
                },
                Token {
                    token_type: TokenType::LPAREN,
                    literal: "("
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "five"
                },
                Token {
                    token_type: TokenType::COMMA,
                    literal: ","
                },
                Token {
                    token_type: TokenType::IDENT,
                    literal: "ten"
                },
                Token {
                    token_type: TokenType::RPAREN,
                    literal: ")"
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::EOF,
                    literal: ""
                },
            ]
        )
    }
}
