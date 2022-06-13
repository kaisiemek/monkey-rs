pub type Instruction = Vec<u8>;

#[derive(Debug)]
pub enum OpcodeType {
    Constant,
}

pub struct OpcodeDefinition {
    name: String,
    bytecode_instruction: u8,
    operand_widths: Vec<u8>,
}

pub fn make(opcode: OpcodeType, operands: Vec<u32>) -> Instruction {
    let operand_widths = opcode.get_operand_widths();
    let instruction_len: u8 = operand_widths.iter().sum::<u8>() + 1;

    let mut instruction: Instruction = Vec::with_capacity(instruction_len as usize);

    instruction.push(opcode.into());
    for (i, operand) in operands.iter().enumerate() {
        let width = operand_widths[i];
        let bytes = match width {
            2 => {
                let value: u16 = operand
                    .to_owned()
                    .try_into()
                    .expect("Could not convert to i16");

                value.to_be_bytes()
            }
            _ => todo!(),
        };
        instruction.extend(bytes.iter().take(width as usize));
    }

    instruction
}

impl From<OpcodeType> for u8 {
    fn from(op: OpcodeType) -> Self {
        op.get_definition().bytecode_instruction
    }
}

impl OpcodeType {
    pub fn get_definition(&self) -> OpcodeDefinition {
        match self {
            OpcodeType::Constant => OpcodeDefinition {
                name: "Constant".to_string(),
                bytecode_instruction: 0x01,
                operand_widths: vec![2],
            },
        }
    }
    pub fn get_operand_widths(&self) -> Vec<u8> {
        self.get_definition().operand_widths
    }
}
