pub mod function;
pub mod lexer;
pub mod interpreter;
pub mod parser;

use lexer::{ Lexer, Token };
use interpreter::{ Interpreter };
use parser::{ Parser };

fn main() {
    let source = "
    (define x 28)
    (+ x 2)".to_string();

    let mut lexer = Lexer::new(source, 0);

    loop {
        let current = lexer.next();

        match current {
            Token::EOF => break,
            _ => println!("Token: {:?}", current),
        }
    }

    let parser = Parser::new(lexer);

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
