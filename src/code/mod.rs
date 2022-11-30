mod test;

macro_rules! make_opcodes {
    ([$($op:ident: $width:expr),+]) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Opcode {
            $($op),+
        }

        impl Opcode {
            pub fn width(&self) -> u8 {
                match self {
                    $(
                        Opcode::$op => $width
                    ),+
                }
            }
        }

        impl ToString for Opcode {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Opcode::$op => stringify!($op).to_string()
                    ),+
                }
            }
        }

        impl Into<u8> for Opcode {
            fn into(self) -> u8 {
                self as u8
            }
        }

        impl TryFrom<u8> for Opcode {
            type Error = String;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                $(
                    if value == Opcode::$op.into() {
                        return Ok(Opcode::$op);
                    }
                )+

                Err("No such opcode".to_string())
            }
        }

    };
}

make_opcodes!([
    Constant: 2,
    Array: 2,
    Hash: 2,
    GetGlobal: 2,
    SetGlobal: 2,
    GetLocal: 2,
    SetLocal: 2,
    Jump: 2,
    JumpNotTruthy: 2,
    Call: 2,
    ReturnValue: 0,
    Return: 0,
    Index: 0,
    Add: 0,
    Sub: 0,
    Mult: 0,
    Div: 0,
    Equal: 0,
    NotEqual: 0,
    GreaterThan: 0,
    Bang: 0,
    Minus: 0,
    Pop: 0,
    True: 0,
    False: 0,
    Null: 0
]);

pub type Instructions = Vec<u8>;

pub fn make(op: Opcode, operands: Vec<u16>) -> Instructions {
    let operand_widths = op.width();

    if operands.len() as u8 != operand_widths / 2 {
        return vec![];
    }

    let mut instruction: Instructions = vec![op.into()];

    match operand_widths {
        0 => {} // nothing to do
        2 => instruction.extend(operands[0].to_be_bytes()),
        _ => todo!(),
    }

    instruction
}

pub fn read_operands(op: Opcode, instructions: &[u8]) -> (Vec<u16>, u16) {
    let mut operands = vec![];
    let mut offset = 0;

    let operands_num = op.width() / 2;
    for _ in 1..=operands_num {
        let operand: u16 = read_u16(&instructions[offset..offset + 2]);
        operands.push(operand);
        offset += 2;
    }

    (operands, offset as u16)
}

pub fn read_u16(instructions: &[u8]) -> u16 {
    ((instructions[0] as u16) << 8) | instructions[1] as u16
}

pub fn stringify(instructions: Instructions) -> Result<String, String> {
    let mut out_str = "".to_string();
    let mut i = 0;

    while i < instructions.len() {
        let op = Opcode::try_from(instructions[i])?;
        let (operands, read_bytes) = read_operands(op, &instructions[(i + 1)..]);
        out_str += &format!("{:0>4} {}\n", i, format_instruction(op, operands)?);
        i += 1 + read_bytes as usize;
    }

    Ok(out_str)
}

fn format_instruction(op: Opcode, operands: Vec<u16>) -> Result<String, String> {
    if (op.width() / 2) as usize != operands.len() {
        return Err(format!(
            "Not enough operants (got: {}, expected: {}) for opcode {} to format",
            operands.len(),
            op.width() / 2,
            op.to_string(),
        ));
    }

    match operands.len() {
        0 => Ok(op.to_string()),
        1 => Ok(format!("{} {}", op.to_string(), operands[0])),
        _ => Err(format!("Unexpected amount of operands: {}", operands.len())),
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::Null
    }
}
