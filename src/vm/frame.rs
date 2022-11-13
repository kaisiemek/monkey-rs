use crate::code::Instructions;

#[derive(Clone)]
pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize,
}

impl Frame {
    pub fn new(instructions: Instructions) -> Self {
        Frame {
            instructions,
            ip: 0,
        }
    }
}
