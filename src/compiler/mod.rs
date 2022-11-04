mod test;

use crate::{code::Instructions, interpreter::object::Object, parser::ast::Program};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {
            instructions: vec![],
            constants: vec![],
        };
    }

    pub fn compile(&self, program: Program) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    pub fn bytecode(self) -> Bytecode {
        return Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        };
    }
}
