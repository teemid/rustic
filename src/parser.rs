use std::collections::{ HashMap };

use lexer::{ Lexer, Token };

type GrammarFunction = fn(&mut Parser);

struct Rule {
    prefix: Option<GrammarFunction>,
    infix: Option<GrammarFunction>,
    postfix: Option<GrammarFunction>,
}

const LiteralRule: Rule = Rule { prefix: Some(Parser::literal), infix: None, postfix: None };

struct Function {
    identifiers: HashMap<String, u32>,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
        }
    }

    pub fn parse(&mut self) {

    }

    fn get_rule(token: Token) -> Rule {
        match token {
            Token::Identifier(_) => LiteralRule,
            Token::Number(_) => LiteralRule,
            _ => panic!("Illegal token"),
        }
    }

    fn literal(&mut self) {
        match self.lexer.current() {
            _ => panic!("Expected token of literal type. Got {:?}", self.lexer.current()),
        }
    }
}
