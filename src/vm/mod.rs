mod test;

use std::{collections::HashMap, iter};

use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    interpreter::object::{Inspectable, Object},
};

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 65536;

pub struct VM {
    instructions: Instructions,
    constants: Vec<Object>,
    stack: Vec<Object>,
    globals: Vec<Object>,
    sp: usize,
}

impl VM {
    pub fn new() -> Self {
        // User iterators to init stack/globals as Object can't implement 'Copy' trait
        // --> no array possible
        VM {
            instructions: vec![],
            constants: vec![],
            stack: iter::repeat(Object::Null).take(STACK_SIZE).collect(),
            globals: iter::repeat(Object::Null).take(GLOBALS_SIZE).collect(),
            sp: 0,
        }
    }

    pub fn run(&mut self, bytecode: Bytecode) -> Result<(), String> {
        // Do not reset the globals (for REPL)
        self.instructions = bytecode.instructions;
        self.constants = bytecode.constants;
        self.sp = 0;
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
                Opcode::Array => {
                    let num_elements = read_u16(&self.instructions[ip..]);
                    ip += 2;

                    let array = self.build_array(self.sp - num_elements as usize, self.sp);
                    self.sp -= num_elements as usize;
                    self.push(array)?;
                }
                Opcode::Hash => {
                    let num_elements = read_u16(&self.instructions[ip..]);
                    ip += 2;

                    let hash = self.build_hash(self.sp - num_elements as usize, self.sp);
                    self.sp -= num_elements as usize;
                    self.push(hash)?;
                }
                Opcode::Add
                | Opcode::Sub
                | Opcode::Mult
                | Opcode::Div
                | Opcode::Equal
                | Opcode::NotEqual
                | Opcode::GreaterThan => {
                    self.execute_binary_op(op)?;
                }
                Opcode::Bang | Opcode::Minus => {
                    self.execute_unary_op(op)?;
                }
                Opcode::Pop => {
                    self.pop()?;
                }
                Opcode::Jump => {
                    let position = read_u16(&self.instructions[ip..]);
                    ip = position as usize;
                }
                Opcode::JumpNotTruthy => {
                    let position = read_u16(&self.instructions[ip..]);
                    ip += 2;

                    let condition = self.pop()?;
                    if !(is_truthy(&condition)) {
                        ip = position as usize;
                    }
                }
                Opcode::True => {
                    self.push(Object::Boolean(true))?;
                }
                Opcode::False => {
                    self.push(Object::Boolean(false))?;
                }
                Opcode::Null => {
                    self.push(Object::Null)?;
                }
                Opcode::GetGlobal => {
                    let index = read_u16(&self.instructions[ip..]);
                    ip += 2;
                    self.push(self.globals[index as usize].clone())?;
                }
                Opcode::SetGlobal => {
                    let index = read_u16(&self.instructions[ip..]);
                    ip += 2;
                    self.globals[index as usize] = self.pop()?;
                }
                Opcode::Index => {
                    let index = self.pop()?;
                    let left = self.pop()?;
                    self.execute_index_expression(left, index)?;
                }
                Opcode::Call => todo!(),
                Opcode::Return => todo!(),
                Opcode::ReturnValue => todo!(),
            }
        }

        Ok(())
    }

    pub fn last_popped_stack_elem(&self) -> Object {
        self.stack[self.sp].clone()
    }

    fn execute_binary_op(&mut self, op: Opcode) -> Result<(), String> {
        let right = self.pop()?;
        let left = self.pop()?;

        if let Object::Integer(left_int) = left {
            if let Object::Integer(right_int) = right {
                match op {
                    Opcode::Add => self.push(Object::Integer(left_int + right_int))?,
                    Opcode::Sub => self.push(Object::Integer(left_int - right_int))?,
                    Opcode::Mult => self.push(Object::Integer(left_int * right_int))?,
                    Opcode::Div => self.push(Object::Integer(left_int / right_int))?,
                    Opcode::Equal => self.push(Object::Boolean(left_int == right_int))?,
                    Opcode::NotEqual => self.push(Object::Boolean(left_int != right_int))?,
                    Opcode::GreaterThan => self.push(Object::Boolean(left_int > right_int))?,
                    _ => {
                        return Err(format!(
                            "Unsupported operation {} for type integer",
                            op.to_string()
                        ))
                    }
                }
                return Ok(());
            }
        } else if let Object::Boolean(left_bool) = left {
            if let Object::Boolean(right_bool) = right {
                match op {
                    Opcode::Equal => self.push(Object::Boolean(left_bool == right_bool))?,
                    Opcode::NotEqual => self.push(Object::Boolean(left_bool != right_bool))?,
                    _ => {
                        return Err(format!(
                            "Unsupported operation {} for type integer",
                            op.to_string()
                        ))
                    }
                }
                return Ok(());
            }
        } else if let Object::String(left_str) = left.clone() {
            if let Object::String(right_str) = right {
                match op {
                    Opcode::Add => self.push(Object::String(left_str + &right_str))?,
                    _ => {
                        return Err(format!(
                            "Unsupported operation {} for type string",
                            op.to_string()
                        ))
                    }
                }
                return Ok(());
            }
        }

        return Err(format!(
            "Unsupported types for {:?}: {} and {}",
            op,
            left.type_str(),
            right.type_str()
        ));
    }

    fn execute_unary_op(&mut self, op: Opcode) -> Result<(), String> {
        let right = self.pop()?;

        match op {
            Opcode::Bang => self.push(Object::Boolean(!is_truthy(&right))),
            Opcode::Minus => {
                if let Object::Integer(right_int) = right {
                    self.push(Object::Integer(-right_int))
                } else {
                    Err(format!(
                        "Unsupported type for unary minus operation: {}",
                        right.type_str()
                    ))
                }
            }
            _ => Err(format!(
                "Unsupported operand type for {}: {}",
                op.to_string(),
                right.type_str()
            )),
        }
    }

    fn execute_index_expression(&mut self, left: Object, index: Object) -> Result<(), String> {
        match left {
            Object::Array(array) => {
                let Object::Integer(i) = index else {
                    return Err("The index for an array must be an integer".to_string());
                };

                if i < 0 || i as usize >= array.len() {
                    self.push(Object::Null)
                } else {
                    self.push(array[i as usize].clone())
                }
            }
            Object::Hash(hash) => match hash.get(&index) {
                Some(val) => self.push(val.clone()),
                None => self.push(Object::Null),
            },
            other => Err(format!(
                "Unsupported type {} for index operator",
                other.type_str()
            )),
        }
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

    fn build_array(&mut self, start: usize, end: usize) -> Object {
        let mut elements: Vec<Object> = iter::repeat(Object::Null).take(end - start).collect();

        for i in start..end {
            elements[i - start] = self.stack[i].clone();
        }

        Object::Array(elements)
    }

    fn build_hash(&mut self, start: usize, end: usize) -> Object {
        let mut hashmap: HashMap<Object, Object> = HashMap::new();

        for i in (start..end).step_by(2) {
            let key = self.stack[i].clone();
            let value = self.stack[i + 1].clone();
            hashmap.insert(key, value);
        }

        Object::Hash(hashmap)
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(val) => *val,
        Object::Null => false,
        _ => true,
    }
}
