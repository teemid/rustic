pub mod function;
pub mod lexer;
pub mod interpreter;

use lexer::{ Lexer, Token };
use interpreter::{ Interpreter };


fn main() {
    let source = "(let id (+ 2 (- 5 1)))".to_string();
    let lexer = Lexer::new(source, 0);

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
