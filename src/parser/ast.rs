use crate::lexer::token::Token;

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

trait Node {
    fn token_literal(&self) -> String;
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        match &self.statements[..] {
            [first, _] => first.token_literal(),
            _ => "".to_owned(),
        }
    }
}

struct LetStatement<'program> {
    token: Token<'program>,
    name: Box<Identifier<'program>>,
    value: dyn Expression,
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) {}
}
impl Node for LetStatement<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

struct Identifier<'program> {
    token: Token<'program>,
    value: String,
}

impl Expression for Identifier<'_> {
    fn expression_node(&self) {}
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}
