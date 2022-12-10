mod frame;
mod test;

use self::frame::Frame;
use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    object::{
        builtins::{get_builtin, BUILTIN_NAMES},
        Inspectable, Object,
    },
};
use std::{collections::HashMap, iter};

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 65536;
const MAX_FRAMES: usize = 1024;

pub struct VM {
    frames: Vec<Frame>,
    stack: Vec<Object>,
    constants: Vec<Object>,
    globals: Vec<Object>,
    last_popped: Object,
}

impl VM {
    pub fn new() -> Self {
        // User iterators to init stack/globals as Object can't implement 'Copy' trait
        // --> no array possible
        VM {
            frames: vec![],
            stack: vec![],
            constants: vec![],
            globals: iter::repeat(Object::Null).take(GLOBALS_SIZE).collect(),
            last_popped: Object::default(),
        }
    }

    pub fn run(&mut self, bytecode: Bytecode) -> Result<&Object, String> {
        self.clear();
        // Put the main function in the call frame
        self.push_frame(Frame::new(bytecode.instructions, 0))?;
        self.constants = bytecode.constants;

        while self.current_frame().instruction_ptr < self.current_frame().instructions.len() {
            let mut frame = self.current_frame().clone();
            let op = Opcode::try_from(frame.instructions[frame.instruction_ptr])?;
            self.current_frame().instruction_ptr += 1;
            frame.instruction_ptr += 1;
            let update_frame = self.execute_op(op, &mut frame)?;
            if update_frame {
                *self.current_frame() = frame;
            }
        }

        Ok(&self.last_popped)
    }

    fn execute_op(&mut self, op: Opcode, frame: &mut Frame) -> Result<bool, String> {
        // println!(
        //     "EXECUTING {}: {}",
        //     frame.instruction_ptr - 1,
        //     op.to_string()
        // );
        // println!("CURRENT STACK:");
        // for (i, obj) in self.stack.iter().enumerate() {
        //     println!("\t{}: {}", i, obj.inspect());
        // }
        match op {
            Opcode::Constant => {
                let const_index =
                    read_u16(&frame.instructions[frame.instruction_ptr..frame.instruction_ptr + 2]);
                self.push_constant(const_index as usize)?;
                frame.instruction_ptr += 2;
            }
            Opcode::Array => {
                let num_elements = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;
                let array = self.build_array(num_elements as usize)?;
                self.push(array)?;
            }
            Opcode::Hash => {
                let num_elements = read_u16(&frame.instructions[frame.instruction_ptr..]) / 2;
                frame.instruction_ptr += 2;
                let hash = self.build_hash(num_elements as usize)?;
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
                self.last_popped = self.pop()?;
            }
            Opcode::Jump => {
                let position = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr = position as usize;
            }
            Opcode::JumpNotTruthy => {
                let position = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;
                let condition = self.pop()?;
                if !(is_truthy(&condition)) {
                    frame.instruction_ptr = position as usize;
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
                let index = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;
                self.push(self.globals[index as usize].clone())?;
            }
            Opcode::SetGlobal => {
                let index = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;
                self.globals[index as usize] = self.pop()?;
            }
            Opcode::Index => {
                let index = self.pop()?;
                let left = self.pop()?;
                self.execute_index_expression(left, index)?;
            }
            Opcode::Call => {
                let num_args = read_u16(&frame.instructions[frame.instruction_ptr..]);
                self.current_frame().instruction_ptr += 2;

                self.call_function(num_args as usize)?;
                return Ok(false);
            }
            Opcode::Return => {
                let base_ptr = self.pop_frame()?.base_ptr;
                while self.stack.len() >= base_ptr {
                    self.pop()?;
                }
                self.push(Object::Null)?;
                return Ok(false);
            }
            Opcode::ReturnValue => {
                let return_value = self.pop()?;
                let base_ptr = self.pop_frame()?.base_ptr;
                while self.stack.len() >= base_ptr {
                    self.pop()?;
                }
                self.push(return_value)?;
                return Ok(false);
            }
            Opcode::GetLocal => {
                let index = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;

                let base_ptr = self.current_frame().base_ptr;
                self.push(self.stack[base_ptr + index as usize].clone())?;
            }
            Opcode::SetLocal => {
                let index = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;

                let base_ptr = self.current_frame().base_ptr;
                self.stack[base_ptr + index as usize] = self.pop()?;
            }
            Opcode::GetBuiltin => {
                let index = read_u16(&frame.instructions[frame.instruction_ptr..]);
                frame.instruction_ptr += 2;

                self.push(Object::BuiltIn {
                    name: BUILTIN_NAMES[index as usize].to_string(),
                })?;
            }
        }
        Ok(true)
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

    fn call_function(&mut self, num_args: usize) -> Result<(), String> {
        let func = self.stack[self.stack.len() - 1 - num_args as usize].clone();

        match func {
            Object::CompiledFunction {
                instructions,
                num_locals,
                num_parameters,
            } => self.call_compiled_function(num_args, instructions, num_locals, num_parameters),
            Object::BuiltIn { name } => self.call_builtin_function(num_args, name),
            _ => Err(format!("Can not call object of type {}", func.type_str())),
        }
    }

    fn call_compiled_function(
        &mut self,
        num_args: usize,
        instructions: Instructions,
        num_locals: usize,
        num_parameters: usize,
    ) -> Result<(), String> {
        if num_parameters != num_args {
            return Err(format!(
                "Wrong number of arguments: want={}, got={}",
                num_parameters, num_args
            ));
        }

        let base_ptr = self.stack.len() - num_args;

        let frame = Frame::new(instructions, base_ptr);
        self.push_frame(frame)?;
        for _ in 1..=base_ptr + num_locals {
            self.stack.push(Object::default());
        }

        Ok(())
    }

    fn call_builtin_function(&mut self, num_args: usize, name: String) -> Result<(), String> {
        let base_ptr = self.stack.len() - num_args;
        let args: Vec<Object> = self.stack[base_ptr..self.stack.len()].to_vec();

        let builtin_func = match get_builtin(&name) {
            Some(func) => func,
            None => return Err(format!("No builtin function named {}", name)),
        };

        let return_value = (builtin_func.func)(args)?;

        while self.stack.len() >= base_ptr {
            self.pop()?;
        }

        self.push(return_value)?;
        Ok(())
    }

    fn push_constant(&mut self, index: usize) -> Result<(), String> {
        match self.constants.get(index) {
            Some(obj) => self.push(obj.clone()),
            None => Err(format!("No constant at index {}", index)),
        }
    }

    fn push(&mut self, obj: Object) -> Result<(), String> {
        if self.stack.len() >= STACK_SIZE {
            return Err(format!("Stack overflow! Exceeded size of {}", STACK_SIZE));
        }
        self.stack.push(obj);
        Ok(())
    }

    fn pop(&mut self) -> Result<Object, String> {
        match self.stack.pop() {
            Some(obj) => Ok(obj),
            None => Err("Stack is empty!".to_string()),
        }
    }

    fn current_frame(&mut self) -> &mut Frame {
        self.frames
            .last_mut()
            .expect("Tried to access an empty stack frame!")
    }

    fn push_frame(&mut self, frame: Frame) -> Result<(), String> {
        if self.frames.len() >= MAX_FRAMES {
            Err(format!(
                "Exceeded maximum number of frames! ({})",
                MAX_FRAMES
            ))
        } else {
            self.frames.push(frame);
            Ok(())
        }
    }

    fn pop_frame(&mut self) -> Result<Frame, String> {
        match self.frames.pop() {
            Some(frame) => Ok(frame),
            None => Err("Tried to pop from an empty stack frame!".to_string()),
        }
    }

    fn build_array(&mut self, num_elements: usize) -> Result<Object, String> {
        let mut elements = vec![];
        for _ in 1..=num_elements {
            elements.push(self.pop()?);
        }
        elements.reverse(); // popping from the stacks results in the reverse order
        Ok(Object::Array(elements))
    }

    fn build_hash(&mut self, num_elements: usize) -> Result<Object, String> {
        let mut hashmap: HashMap<Object, Object> = HashMap::new();
        for _ in 1..=num_elements {
            let value = self.pop()?;
            let key = self.pop()?;
            hashmap.insert(key, value);
        }

        Ok(Object::Hash(hashmap))
    }

    fn clear(&mut self) {
        // Do not reset the globals (for REPL)
        self.frames.clear();
        self.constants.clear();
        self.stack.clear();
        self.last_popped = Object::default();
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(val) => *val,
        Object::Null => false,
        _ => true,
    }
}
