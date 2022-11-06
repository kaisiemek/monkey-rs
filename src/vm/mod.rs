mod test;
use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    interpreter::object::Object,
};

const STACK_SIZE: usize = 2048;

pub struct VM {
    constants: Vec<Object>,
    instructions: Instructions,
    stack: Vec<Object>,
    sp: usize,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        // Can't use an array as Object can't implement Copy
        let mut stack = Vec::with_capacity(STACK_SIZE);
        for _ in 1..=STACK_SIZE {
            stack.push(Object::Null);
        }

        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack,
            sp: 0,
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
                Opcode::Add | Opcode::Sub | Opcode::Mult | Opcode::Div => {
                    self.execute_binary_op(op)?;
                }
                Opcode::Pop => {
                    self.pop()?;
                }
            }
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Object {
        if self.sp < 1 {
            Object::Null
        } else {
            self.stack[self.sp - 1].clone()
        }
    }

    pub fn last_popped_stack_elem(&self) -> Object {
        self.stack[self.sp].clone()
    }

    fn push_constant(&mut self, index: usize) -> Result<(), String> {
        if index >= self.constants.len() {
            return Err(format!("No constant at index {}", index));
        }

        self.push(self.constants[index].clone())
    }

    fn push(&mut self, obj: Object) -> Result<(), String> {
        if self.sp >= STACK_SIZE {
            return Err(format!("Stack overflow! Exceeded size of {}", STACK_SIZE));
        }

        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<Object, String> {
        if self.sp < 1 {
            return Err("Stack is empty!".to_string());
        }

        let obj = self.stack[self.sp - 1].clone();
        self.sp -= 1;
        return Ok(obj);
    }

    fn execute_binary_op(&mut self, op: Opcode) -> Result<(), String> {
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

        match op {
            Opcode::Add => self.push(Object::Integer(left + right))?,
            Opcode::Sub => self.push(Object::Integer(left - right))?,
            Opcode::Mult => self.push(Object::Integer(left * right))?,
            Opcode::Div => self.push(Object::Integer(left / right))?,
            _ => panic!("unreachable"),
        }

        Ok(())
    }
}
