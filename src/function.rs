use std::collections::{ HashMap };
use std::ops::Sub;

#[derive(Debug)]
pub struct Function {
    instructions: Vec<u32>,
    constants: Vec<u32>,
    identifiers: HashMap<String, usize>,
}

impl Function {
    pub fn add_constant(&mut self, number: u32) -> usize {
        self.constants.push(number);

        let index = self.constants.len() - 1;

        self.constants[index];

        index
    }

    pub fn add_identifier(&mut self, identifier: String) -> usize {
        let index = self.identifiers.len();

        self.identifiers.insert(identifier, index);

        index
    }

    pub fn get_instruction(&mut self, index: usize) -> u32 {
        self.instructions[index]
    }
}