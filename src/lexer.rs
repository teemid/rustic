#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    // NOTE (Emil): I don't know if this is the best way of doing things but let's try.
    Let,
    Quote,
    EOF,
}

#[derive(Debug)]
pub struct Lexer {
    pos: usize,
    input: String,
}

impl Lexer {
    pub fn new(input: String, pos: usize) -> Lexer {
        Lexer {
            pos: pos,
            input: input
        }
    }

    pub fn next(&mut self) -> Token {
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
            '\'' => Token::Quote,
            'a' ... 'z' | 'A' ... 'Z' | '_' => self.identifier(c),
            '0' ... '9' => self.number(c),
            _ => panic!("Invalid character {}", c),
        }
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut identifier = first.to_string();

        identifier += &self.consume_while(|c| match c {
            'a' ... 'z' | 'A' ... 'Z' | '_' => true,
            _ => false,
        });

        match identifier.as_ref() {
            "let" => Token::Let,
            _ => Token::Identifier(identifier),
        }
    }

    fn number(&mut self, first: char) -> Token {
        let mut number = first.to_string();

        number += &self.consume_while(|c| match c {
            '0' ... '9' => true,
            _ => false,
        });

        return Token::Number(number.parse().unwrap());
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

        return result;
    }

    fn consume_char(&mut self) -> char {
        let mut iterator = self.input[self.pos ..].char_indices();

        let (_, cur_char) = iterator.next().unwrap();
        let (next_pos, _) = iterator.next().unwrap_or((1, ' '));

        self.pos += next_pos;

        return cur_char;
    }

    fn next_char(&self) -> char {
        self.input[self.pos ..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}