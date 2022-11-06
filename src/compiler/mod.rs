mod test;

use crate::{
    code::{make, Instructions, Opcode},
    interpreter::object::Object,
    parser::ast::{Expression, Program, Statement},
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
    last_instruction: EmittedInstruction,
    previous_instruction: EmittedInstruction,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

#[derive(Clone)]
struct EmittedInstruction {
    pub opcode: Opcode,
    pub position: usize,
}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {
            instructions: vec![],
            constants: vec![],
            last_instruction: EmittedInstruction {
                opcode: Opcode::Pop,
                position: 0,
            },
            previous_instruction: EmittedInstruction {
                opcode: Opcode::Pop,
                position: 0,
            },
        };
    }

    pub fn compile(&mut self, program: Program) -> Result<(), String> {
        for statement in program {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    pub fn bytecode(self) -> Bytecode {
        return Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        };
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Let {
                token,
                identifier,
                value,
            } => todo!(),
            Statement::Return { token, value } => todo!(),
            Statement::Expression {
                token: _,
                expression,
            } => {
                self.compile_expression(expression)?;
                self.emit(Opcode::Pop, vec![]);
                Ok(())
            }
        }
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), String> {
        match expression {
            Expression::Identifier { token, value } => todo!(),
            Expression::IntLiteral { token: _, value } => {
                let integer = Object::Integer(value);
                let constant_idx = self.add_constant(integer);
                self.emit(Opcode::Constant, vec![constant_idx as u16]);
            }
            Expression::BoolLiteral { token: _, value } => {
                let opcode = if value { Opcode::True } else { Opcode::False };
                self.emit(opcode, vec![]);
            }
            Expression::StringLiteral { token, value } => todo!(),
            Expression::ArrayLiteral { token, elements } => todo!(),
            Expression::HashLiteral { token, entries } => todo!(),
            Expression::FnLiteral {
                token,
                parameters,
                body,
            } => todo!(),
            Expression::Prefix {
                token: _,
                operator,
                right_expression,
            } => {
                self.compile_prefix_expression(operator, *right_expression)?;
            }
            Expression::Infix {
                token: _,
                left_expression,
                operator,
                right_expression,
            } => {
                self.compile_infix_expression(*left_expression, operator, *right_expression)?;
            }
            Expression::Index { token, left, index } => todo!(),
            Expression::If {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                self.compile_expression(*condition)?;
                // 0xFFFF placeholder value, will be replaced later on
                let jump_pos = self.emit(Opcode::JumpNotTruthy, vec![0xFFFF]);
                self.compile(consequence.statements)?;
                if self.last_instruction.opcode == Opcode::Pop {
                    self.remove_last_pop();
                }

                let after_consequence_pos = self.instructions.len();
                self.change_operand(jump_pos, after_consequence_pos as u16);
            }
            Expression::Call {
                token,
                function,
                arguments,
            } => todo!(),
        }

        Ok(())
    }

    fn compile_infix_expression(
        &mut self,
        left: Expression,
        op: String,
        right: Expression,
    ) -> Result<(), String> {
        // reorder operators if it's lesser than
        if op == "<" {
            self.compile_expression(right)?;
            self.compile_expression(left)?;
        } else {
            self.compile_expression(left)?;
            self.compile_expression(right)?;
        }

        let opcode = match op.as_str() {
            "+" => Opcode::Add,
            "-" => Opcode::Sub,
            "*" => Opcode::Mult,
            "/" => Opcode::Div,
            "<" | ">" => Opcode::GreaterThan,
            "==" => Opcode::Equal,
            "!=" => Opcode::NotEqual,
            other => return Err(format!("Unknown operator: {}", other)),
        };

        self.emit(opcode, vec![]);
        Ok(())
    }

    fn compile_prefix_expression(&mut self, op: String, right: Expression) -> Result<(), String> {
        self.compile_expression(right)?;

        let opcode = match op.as_str() {
            "!" => Opcode::Bang,
            "-" => Opcode::Minus,
            _ => {
                return Err(format!("Unsupported operator for prefix operation: {}", op));
            }
        };

        self.emit(opcode, vec![]);
        Ok(())
    }

    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        return self.constants.len() - 1;
    }

    fn remove_last_pop(&mut self) {
        let last_instr_pos = self.last_instruction.position;
        self.instructions.truncate(last_instr_pos);
        self.last_instruction = self.previous_instruction.clone();
    }

    fn change_operand(&mut self, op_pos: usize, operand: u16) {
        let opcode: Opcode = self.instructions[op_pos].try_into().unwrap();
        let new_instruction = make(opcode, vec![operand]);
        self.replace_instruction(op_pos, new_instruction);
    }

    fn replace_instruction(&mut self, pos: usize, new_instruction: Instructions) {
        let end_pos = pos + new_instruction.len();
        self.instructions
            .splice(pos..end_pos, new_instruction.iter().cloned());
    }

    fn emit(&mut self, op: Opcode, operands: Vec<u16>) -> usize {
        let instruction = make(op, operands);
        let instruction_pos = self.instructions.len();
        self.instructions.extend(instruction);

        self.previous_instruction = self.last_instruction.clone();
        self.last_instruction = EmittedInstruction {
            opcode: op,
            position: instruction_pos,
        };

        return instruction_pos;
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
