use std::fmt::format;

use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    interpreter::object::Object,
};

mod test;

const STACK_SIZE: usize = 2048;

pub struct VM {
    constants: Vec<Object>,
    instructions: Instructions,
    stack: Vec<Object>,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip: usize = 0;
        while ip < self.instructions.len() {
            let op = Opcode::try_from(self.instructions[ip])?;
            ip += 1;

            match op {
                Opcode::Constant => {
                    let const_index = read_u16(&self.instructions[ip..ip + 2]);
                    self.push_constant(const_index as usize)?;
                    ip += 2;
                }
                Opcode::Add => {
                    let right = match self.pop()? {
                        Object::Integer(i) => i,
                        other => {
                            return Err(format!(
                                "Unexpected right operand for add instruction: {:?}",
                                other
                            ))
                        }
                    };

                    let left = match self.pop()? {
                        Object::Integer(i) => i,
                        other => {
                            return Err(format!(
                                "Unexpected left operand for add instruction: {:?}",
                                other
                            ))
                        }
                    };

                    self.push(Object::Integer(left + right))?;
                }
            }
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Object {
        if self.stack.is_empty() {
            Object::Null
        } else {
            self.stack.last().unwrap().clone()
        }
    }

    fn push_constant(&mut self, index: usize) -> Result<(), String> {
        if index >= self.constants.len() {
            return Err(format!("No constant at index {}", index));
        }

        self.push(self.constants[index].clone())
    }

    fn push(&mut self, obj: Object) -> Result<(), String> {
        if self.stack.len() >= STACK_SIZE {
            return Err(format!("Stack overflow! Exceeded size of {}", STACK_SIZE));
        }

        self.stack.push(obj);
        Ok(())
    }

    fn pop(&mut self) -> Result<Object, String> {
        let obj = self.stack.pop();
        match obj {
            Some(obj) => Ok(obj),
            None => Err("Stack is empty!".to_string()),
        }
    }
}
