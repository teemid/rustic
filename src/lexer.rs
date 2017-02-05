#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(i64),
    String(String),
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    // NOTE (Emil): I don't know if this is the best way of doing things but let's try.
    And,
    Case,
    Cons,
    Define,
    Else,
    If,
    Lambda,
    Let,
    Or,
    Quote,
    EOF,
}

const AND_KEYWORD_STR:    &'static str = "and";
const CASE_KEYWORD_STR:   &'static str = "case";
const CONS_KEYWORD_STR:   &'static str = "cons";
const DEFINE_KEYWORD_STR: &'static str = "define";
const ELSE_KEYWORD_STR:   &'static str = "else";
const IF_KEYWORD_STR:     &'static str = "if";
const LAMBDA_KEYWORD_STR: &'static str = "lambda";
const LET_KEYWORD_STR:    &'static str = "let";
const OR_KEYWORD_STR:     &'static str = "or";
const QUOTE_KEYWORD_STR:  &'static str = "quote";

#[derive(Debug)]
pub struct Lexer {
    pos: usize,
    input: String,
    current: Token,
    lookahead: Token,
}

impl Lexer {
    pub fn new(input: String, pos: usize) -> Lexer {
        Lexer {
            pos: pos,
            input: input,
            current: Token::EOF,
            // TODO (Emil): Should this be an Option<Token>?
            lookahead: Token::EOF,
        }
    }

    pub fn current(&self) -> Token {
        self.current.clone()
    }

    pub fn peek(&mut self) -> Token {
        let current = self.current();
        self.lookahead = self.next();
        self.current = current;

        self.lookahead.clone()
    }

    pub fn next(&mut self) -> Token {
        if self.lookahead != Token::EOF {
            self.current = self.lookahead.clone();
            self.lookahead = Token::EOF;
        }

        if self.eof() {
            return Token::EOF;
        }

        self.consume_whitespace();

        let c = self.consume_char();

        match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '\'' => Token::Quote,
            '"' => self.string(),
            'a' ... 'z' | 'A' ... 'Z' | '_' => self.identifier(c),
            '0' ... '9' => self.number(c),
            _ => panic!("Invalid character {}", c),
        }
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut identifier = first.to_string();

        identifier += &self.consume_while(|c| match c {
            'a' ... 'z' | 'A' ... 'Z' | '_' | '-' => true,
            _ => false,
        });

        match identifier.as_ref() {
            AND_KEYWORD_STR    => Token::And,
            CASE_KEYWORD_STR   => Token::Case,
            CONS_KEYWORD_STR   => Token::Cons,
            DEFINE_KEYWORD_STR => Token::Define,
            ELSE_KEYWORD_STR   => Token::Else,
            IF_KEYWORD_STR     => Token::If,
            LAMBDA_KEYWORD_STR => Token::Lambda,
            LET_KEYWORD_STR    => Token::Let,
            OR_KEYWORD_STR     => Token::Or,
            QUOTE_KEYWORD_STR  => Token::Quote,
            _ => Token::Identifier(identifier),
        }
    }

    fn number(&mut self, first: char) -> Token {
        let mut number = first.to_string();

        number += &self.consume_while(|c| match c {
            '0' ... '9' => true,
            _ => false,
        });

        // TODO (Emil): Is there any need for anything more than an unwrap here?
        // We have already matched against only number characters.
        Token::Number(number.parse().unwrap())
    }

    fn string(&mut self) -> Token {
        let mut string = self.consume_while(|c| match c {
            '"' => false,
            _ => true,
        });

        Token::String(string.to_string())
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F>(&mut self, test: F) -> String
            where F: Fn(char) -> bool {
        let mut result = String::new();

        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    fn consume_char(&mut self) -> char {
        let mut iterator = self.input[self.pos ..].char_indices();

        let (_, cur_char) = iterator.next().unwrap();
        let (next_pos, _) = iterator.next().unwrap_or((1, ' '));

        self.pos += next_pos;

        cur_char
    }

    fn next_char(&self) -> char {
        self.input[self.pos ..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}