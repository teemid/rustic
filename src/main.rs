pub mod lexer;

use lexer::{ Lexer, Token };

fn main() {
    let source = "(+ 2 2)".to_string();
    let mut lexer = Lexer::new(source, 0);

    loop {
        let current = lexer.next();

        if current == Token::EOF {
            break;
        }

        match current {
            Token::OpenParen => println!("Token: ("),
            Token::CloseParen => println!("Token: )"),
            Token::Number { i } => println!("Token: Number({})", i),
            Token::Plus => println!("Token: +"),
            Token::Minus => println!("Token: -"),
            _ => println!("Unknown token"),
        }
    }
}
