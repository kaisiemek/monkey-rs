macro_rules! make_opcodes {
    ([$($op:ident: $width:expr),+]) => {
        #[repr(u8)]
        // #[derive(Debug, Clone, Copy, PartialEq)]
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
make_opcodes!([Constant: 2]);

pub type Instruction = Vec<u8>;

pub fn make(code: Opcode, operands: Vec<u16>) -> Instruction {
    let operand_widths = code.width();

    if operands.len() as u8 != operand_widths / 2 {}

    let mut instruction: Instruction = vec![code.into()];

    match operand_widths {
        2 => {
            instruction.extend(operands[0].to_be_bytes());
        }
        _ => todo!(),
    }

    instruction
}
