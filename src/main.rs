pub mod function;
pub mod lexer;
pub mod interpreter;
pub mod parser;

use lexer::{ Lexer, Token };
use interpreter::{ Interpreter };


fn main() {
    let source = "(+ 1 2 3 4)".to_string();
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

    let mut interpreter = Interpreter::new();

    interpreter.add_constant(1);
    interpreter.add_constant(2);

    interpreter.add_instruction(0);
    interpreter.add_instruction(0);
    interpreter.add_instruction(0);
    interpreter.add_instruction(1);
    interpreter.add_instruction(3);

    interpreter.run();
}
