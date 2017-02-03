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
    End,
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
    pub fn new() -> Interpreter {
        Interpreter {
            constants: vec![],
            instruction_pointer: 0,
            stack: vec![],
            local_variables: vec![],
            instructions: vec![]
        }
    }

    pub fn add_constant(&mut self, constant: u32) {
        self.constants.push(constant);
    }

    pub fn add_instruction(&mut self, instruction: u32) {
        self.instructions.push(instruction);
    }

    pub fn run(&mut self) {
        loop {
            println!("{:?}", self.stack);

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
                Instruction::End            => break,
            }        
        }
    }

    fn next_instruction(&mut self) -> Instruction {
        match self.next() {
            0 => Instruction::LoadConstant,
            1 => Instruction::Add,
            2 => Instruction::Subtract,
            3 => Instruction::Multiply,
            4 => Instruction::Divide,
            5 => Instruction::Jump,
            6 => Instruction::JumpIfEqual,
            7 => Instruction::JumpIfNotEqual,
            8 => Instruction::End,
            _ => panic!("Illegal instruction"),
        }
    }

    fn next_value(&mut self) -> u32 {
        let value = self.instructions[self.instruction_pointer];

        self.instruction_pointer += 1;

        value
    }

    fn load_constant(&mut self) {
        let index = self.next();

        match self.constants.get(index as usize) {
            Some(constant) => self.stack.push(constant.clone()),
            None => panic!("Failed to load constant"),
        }
    }

    fn load_local(&mut self) {
        let index = self.next();

        self.stack.push(self.constants[index as usize]);
    }

    fn store_local(&mut self) {
        let value = self.pop();
        let index = self.next();

        self.local_variables[index as usize] = value;
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

        self.instruction_pointer = target as usize;
    }

    fn jump_if_equal(&mut self) {
        
    }

    fn jump_if_not_equal(&mut self) {

    }

    fn next(&mut self) -> u32 {
        let instruction = self.instructions.get(self.instruction_pointer);
        self.instruction_pointer += 1;

        match instruction {
            Some(instruction) => instruction.clone(),
            None => 8
        }
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
