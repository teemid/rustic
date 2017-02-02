use function::{ Function };

#[derive(Debug)]
pub enum Instruction {
    LoadConstant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Jump,
    JumpIfEqual,
    JumpIfNotEqual,
}

#[derive(Debug)]
struct Interpreter {
    instruction_pointer: usize,
    executing_function: Function,
}

impl Interpreter {
    fn run(&mut self) {
        loop {
        }
    }

    fn fetch_instruction(&mut self) -> Instruction {
        match self.executing_function.get_instruction(self.instruction_pointer) {
            0 => Instruction::LoadConstant,
            1 => Instruction::Add,
            2 => Instruction::Subtract,
            3 => Instruction::Multiply,
            4 => Instruction::Divide,
            5 => Instruction::Jump,
            6 => Instruction::JumpIfEqual,
            7 => Instruction::JumpIfNotEqual,
            _ => panic!("Illegal instruction"),
        }
    }
}
