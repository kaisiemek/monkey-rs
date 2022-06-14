mod test;

use crate::{
    interpreter::object::Object, parser::ast::Program, virtualmachine::code::Instructions,
};

pub struct Compiler {
    instructions: Vec<Instructions>,
    constants: Vec<Object>,
}

pub struct Bytecode {
    instructions: Instructions,
    constants: Vec<Object>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn compile_program(&self, program: Program) -> Result<(), String> {
        Ok(())
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}
