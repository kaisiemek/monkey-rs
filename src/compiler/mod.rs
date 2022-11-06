mod test;

use crate::{
    code::{make, Instructions, Opcode},
    interpreter::object::Object,
    parser::ast::{Expression, Program, Statement},
};

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
                if value {
                    self.emit(Opcode::True, vec![]);
                } else {
                    self.emit(Opcode::False, vec![]);
                }
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
                token,
                operator,
                right_expression,
            } => todo!(),
            Expression::Infix {
                token: _,
                left_expression,
                operator,
                right_expression,
            } => {
                self.compile_expression(*left_expression)?;
                self.compile_expression(*right_expression)?;

                match operator.as_str() {
                    "+" => {
                        self.emit(Opcode::Add, vec![]);
                    }
                    "-" => {
                        self.emit(Opcode::Sub, vec![]);
                    }
                    "*" => {
                        self.emit(Opcode::Mult, vec![]);
                    }
                    "/" => {
                        self.emit(Opcode::Div, vec![]);
                    }
                    _ => return Err(format!("Unknown operator: {}", operator)),
                }
            }
            Expression::Index { token, left, index } => todo!(),
            Expression::If {
                token,
                condition,
                consequence,
                alternative,
            } => todo!(),
            Expression::Call {
                token,
                function,
                arguments,
            } => todo!(),
        }

        Ok(())
    }

    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        return self.constants.len() - 1;
    }

    fn emit(&mut self, op: Opcode, operands: Vec<u16>) -> usize {
        let instruction = make(op, operands);
        let instruction_pos = self.instructions.len();
        self.instructions.extend(instruction);

        return instruction_pos;
    }
}
