pub mod lexer;

use lexer::{ Lexer, Token };

fn main() {
    let source = "(let id (+ 2 (- 5 1)))".to_string();
    let mut lexer = Lexer::new(source, 0);

    loop {
        let current = lexer.next();

        if current == Token::EOF {
            break;
        }

        match current {
            Token::Identifier(identifier) => println!("Token: Identifier({})", identifier),
            Token::Let => println!("Token: Keyword Let"),
            Token::OpenParen => println!("Token: ("),
            Token::CloseParen => println!("Token: )"),
            Token::Number(number) => println!("Token: Number({})", number),
            Token::Plus => println!("Token: +"),
            Token::Minus => println!("Token: -"),
            _ => println!("Unknown token"),
        }
    }
}
