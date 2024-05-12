mod token;

use core::panic;

use token::Token;
pub struct Lexer<'lexing> {
    input: &'lexing str,
}

impl<'lexing> Lexer<'lexing> {
    pub fn new(code: &'lexing str) -> Self {
        Lexer { input: code }
    }

    pub fn next_token(self) -> (Token<'lexing>, Lexer<'lexing>) {
        let lexer = self.skip_whitespace();
        if let Some(c) = lexer.input.chars().next() {
            let advanced_lexer = Lexer::new(&lexer.input[1..]);
            let (token, lexer) = match c {
                '=' => (Token::ASSIGN, advanced_lexer),
                '+' => (Token::PLUS, advanced_lexer),
                ',' => (Token::COMMA, advanced_lexer),
                ';' => (Token::SEMICOLON, advanced_lexer),
                '(' => (Token::LPAREN, advanced_lexer),
                ')' => (Token::RPAREN, advanced_lexer),
                '{' => (Token::LBRACE, advanced_lexer),
                '}' => (Token::RBRACE, advanced_lexer),
                '0' => (Token::EOF, advanced_lexer),
                x if x.is_ascii_alphabetic() => {
                    let ident_name_len: usize =
                        lexer.input.chars().take_while(is_valid_ident_char).count();
                    let ident_name = &lexer.input[..ident_name_len];
                    let advanced_lexer = Lexer::new(&lexer.input[ident_name_len..]);
                    let token_match: Token = KEYWORD_TOKENS
                        .iter()
                        .filter(|(word, _)| *word == ident_name)
                        .map(|t| t.1.clone())
                        .next()
                        .unwrap_or(Token::IDENT(ident_name));
                    (token_match, advanced_lexer)
                }
                '0'..='9' => {
                    let integer_len = lexer
                        .input
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .count();
                    (
                        Token::INT(&lexer.input[..integer_len]),
                        Lexer::new(&lexer.input[integer_len..]),
                    )
                }
                _ => (Token::ILLEGAL(&lexer.input[..1]), advanced_lexer),
            };
            (token, lexer)
        } else {
            (Token::EOF, Lexer::new(""))
        }
    }

    fn skip_whitespace(self) -> Self {
        Lexer::new(&self.input[self.input.chars().take_while(is_whitespace).count()..])
    }
}

const KEYWORD_TOKENS: [(&str, Token); 2] = [("fn", Token::FUNCTION), ("let", Token::LET)];

fn is_whitespace(c: &char) -> bool {
    *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r'
}

fn is_valid_ident_char(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_'
}

pub fn lex(code: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(code);
    let mut token;
    let mut token_vec: Vec<Token> = vec![];
    loop {
        (token, lexer) = lexer.next_token();
        token_vec.push(token);
        match token_vec.last().unwrap() {
            Token::EOF => return token_vec,
            Token::ILLEGAL(c) => panic!("Illegal token encountered, char {c:?}"),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;
    #[test]
    fn test_next_single_literal_token() {
        const INPUT: &str = "=+(){},;";
        let token_vec = lex(INPUT);
        assert_eq!(
            token_vec,
            vec![
                Token::ASSIGN,
                Token::PLUS,
                Token::LPAREN,
                Token::RPAREN,
                Token::LBRACE,
                Token::RBRACE,
                Token::COMMA,
                Token::SEMICOLON,
                Token::EOF,
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
                Token::LET,
                Token::IDENT("five"),
                Token::ASSIGN,
                Token::INT("5"),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT("ten"),
                Token::ASSIGN,
                Token::INT("10"),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT("add"),
                Token::ASSIGN,
                Token::FUNCTION,
                Token::LPAREN,
                Token::IDENT("x"),
                Token::COMMA,
                Token::IDENT("y"),
                Token::RPAREN,
                Token::LBRACE,
                Token::IDENT("x"),
                Token::PLUS,
                Token::IDENT("y"),
                Token::SEMICOLON,
                Token::RBRACE,
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT("result"),
                Token::ASSIGN,
                Token::IDENT("add"),
                Token::LPAREN,
                Token::IDENT("five"),
                Token::COMMA,
                Token::IDENT("ten"),
                Token::RPAREN,
                Token::SEMICOLON,
                Token::EOF,
            ]
        )
    }
}
