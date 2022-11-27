use crate::code::Instructions;

#[derive(Clone)]
pub struct Frame {
    pub instructions: Instructions,
    pub instruction_ptr: usize,
    pub base_ptr: usize,
}

impl Frame {
    pub fn new(instructions: Instructions, base_ptr: usize) -> Self {
        Frame {
            instructions,
            instruction_ptr: 0,
            base_ptr,
        }
    }
}
