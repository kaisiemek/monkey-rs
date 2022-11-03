mod test;

use crate::{interpreter::object::Object, parser::ast::Program, virtualmachine::code::Instruction};

pub struct Compiler {
    instructions: Instruction,
    constants: Vec<Object>,
}

pub struct Bytecode {
    pub instructions: Instruction,
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
