use function::{ Function };

#[derive(Debug)]
pub enum Instruction {
    LoadConstant,
    LoadLocal,
    StoreLocal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Jump,
    JumpIfEqual,
    JumpIfNotEqual,
}

#[derive(Debug)]
pub struct Interpreter {
    instruction_pointer: usize,
    instructions: Vec<u32>,
    stack: Vec<u32>,
    constants: Vec<u32>,
    local_variables: Vec<u32>,
}

impl Interpreter {
    pub fn run(&mut self) {
        loop {
            match self.next_instruction() {
                Instruction::LoadConstant   => self.load_constant(),
                Instruction::LoadLocal      => self.load_local(),
                Instruction::StoreLocal     => self.store_local(),
                Instruction::Add            => self.add(),
                Instruction::Subtract       => self.subtract(),
                Instruction::Multiply       => self.multiply(),
                Instruction::Divide         => self.divide(),
                Instruction::Jump           => self.do_jump(),
                Instruction::JumpIfEqual    => self.jump_if_equal(),
                Instruction::JumpIfNotEqual => self.jump_if_not_equal(),
                // _ => panic!("Unknown instruction"),
            }        
        }
    }

    fn next_instruction(&mut self) -> Instruction {
        let instruction = match self.instructions[self.instruction_pointer] {
            0 => Instruction::LoadConstant,
            1 => Instruction::Add,
            2 => Instruction::Subtract,
            3 => Instruction::Multiply,
            4 => Instruction::Divide,
            5 => Instruction::Jump,
            6 => Instruction::JumpIfEqual,
            7 => Instruction::JumpIfNotEqual,
            _ => panic!("Illegal instruction"),
        };

        self.instruction_pointer += 1;

        instruction
    }

    fn next_value(&mut self) -> u32 {
        let value = self.instructions[self.instruction_pointer];

        self.instruction_pointer += 1;

        value
    }

    fn load_constant(&mut self) {
        let index = self.pop();

        match self.constants.get(index as usize) {
            Some(constant) => self.stack.push(constant.clone()),
            None => panic!("Failed to load constant"),
        }
    }

    fn load_local(&self) {
    }

    fn store_local(&mut self) {
        let value = self.pop();
        let index = self.pop();
    }

    fn add(&mut self) {
        let a = self.pop();
        let b = self.pop();

        self.push(a + b);
    }

    fn subtract(&mut self) {
        let a = self.pop();
        let b = self.pop();

        self.push(b - a);
    }

    fn multiply(&mut self) {
        let a = self.pop();
        let b = self.pop();

        self.push(a * b);
    }

    fn divide(&mut self) {
        let a = self.pop();
        let b = self.pop();

        self.push(b / a);
    }

    fn do_jump(&mut self) {
        let target = self.pop();
    }

    fn jump_if_equal(&mut self) {

    }

    fn jump_if_not_equal(&mut self) {

    }

    fn pop(&mut self) -> u32 {
        match self.stack.pop() {
            Some(x) => x,
            None => panic!("Tried to pop an empty stack"),
        }
    }

    fn push(&mut self, value: u32) {
        self.stack.push(value);
    }
}
