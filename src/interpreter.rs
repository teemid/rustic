type Instruction = u32;

const LOAD_CONSTANT:     Instruction = 0;
const LOAD_LOCAL:        Instruction = 1;
const STORE_LOCAL:       Instruction = 2;
const ADD:               Instruction = 3;
const SUBTRACT:          Instruction = 4;
const MULTIPLY:          Instruction = 5;
const DIVIDE:            Instruction = 6;
const JUMP:              Instruction = 7;
const JUMP_IF_EQUAL:     Instruction = 8;
const JUMP_IF_NOT_EQUAL: Instruction = 9;
const END:               Instruction = 10;

const INSTRUCTION_NAMES: [&'static str; 11] = [
    "LoadConstant",
    "LoadLocal",
    "StoreLocal",
    "Add",
    "Subtract",
    "Multiply",
    "Divide",
    "Jump",
    "JumpIfEqual",
    "JumpIfNotEqual",
    "End",
];

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

            match self.next() as Instruction {
                LOAD_CONSTANT     => self.load_constant(),
                LOAD_LOCAL        => self.load_local(),
                STORE_LOCAL       => self.store_local(),
                ADD               => self.add(), 
                SUBTRACT          => self.subtract(),
                MULTIPLY          => self.multiply(),
                DIVIDE            => self.divide(),
                JUMP              => self.jump(),
                JUMP_IF_EQUAL     => self.jump_if_equal(),
                JUMP_IF_NOT_EQUAL => self.jump_if_not_equal(),
                END               => break,
                _ => panic!("Illegal instruction"),
            }        
        }
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

    fn jump(&mut self) {
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
            None => END,
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
