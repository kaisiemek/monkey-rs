mod symbol_table;
mod test;

use std::rc::Rc;

use crate::{
    code::{make, Instructions, Opcode},
    interpreter::object::Object,
    parser::ast::{Expression, Program, Statement},
};

use self::symbol_table::{Symbol, SymbolTable};

pub struct Compiler {
    scopes: Vec<CompilationScope>,
    scope_index: usize,
    constants: Vec<Object>,
    symbol_table: SymbolTable,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

#[derive(Clone, Default)]
struct EmittedInstruction {
    pub opcode: Opcode,
    pub position: usize,
}

#[derive(Default)]
struct CompilationScope {
    pub instructions: Instructions,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scopes: vec![CompilationScope::default()],
            scope_index: 0,
            constants: vec![],
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn compile(&mut self, program: Program) -> Result<(), String> {
        self.clear();
        self.compile_program(program)
    }

    pub fn bytecode(&self) -> Bytecode {
        return Bytecode {
            instructions: self.scopes[self.scope_index].instructions.clone(),
            constants: self.constants.clone(),
        };
    }

    fn compile_program(&mut self, program: Program) -> Result<(), String> {
        for statement in program {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Let {
                token: _,
                identifier,
                value,
            } => {
                self.compile_expression(value)?;
                let symbol = self.add_symbol(&identifier);
                let opcode = match symbol.scope {
                    symbol_table::SymbolScope::Global => Opcode::SetGlobal,
                    symbol_table::SymbolScope::Local => Opcode::SetLocal,
                };

                self.emit(opcode, vec![symbol.index as u16]);
            }
            Statement::Return { token: _, value } => {
                self.compile_expression(value)?;
                self.emit(Opcode::ReturnValue, vec![]);
            }
            Statement::Expression {
                token: _,
                expression,
            } => {
                self.compile_expression(expression)?;
                self.emit(Opcode::Pop, vec![]);
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), String> {
        match expression {
            Expression::Identifier { token: _, value } => {
                let symbol = self.resolve_symbol(&value)?;
                let opcode = match symbol.scope {
                    symbol_table::SymbolScope::Global => Opcode::GetGlobal,
                    symbol_table::SymbolScope::Local => Opcode::GetLocal,
                };

                self.emit(opcode, vec![symbol.index as u16]);
            }
            Expression::IntLiteral { token: _, value } => {
                let integer = Object::Integer(value);
                let constant_idx = self.add_constant(integer);
                self.emit(Opcode::Constant, vec![constant_idx as u16]);
            }
            Expression::BoolLiteral { token: _, value } => {
                let opcode = if value { Opcode::True } else { Opcode::False };
                self.emit(opcode, vec![]);
            }
            Expression::StringLiteral { token: _, value } => {
                let string = Object::String(value);
                let constant_idx = self.add_constant(string);
                self.emit(Opcode::Constant, vec![constant_idx as u16]);
            }
            Expression::ArrayLiteral { token: _, elements } => {
                let num_elements = elements.len() as u16;
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.emit(Opcode::Array, vec![num_elements]);
            }
            Expression::HashLiteral { token: _, entries } => {
                let entry_num = entries.len() as u16 * 2;
                for (key, val) in entries {
                    self.compile_expression(key)?;
                    self.compile_expression(val)?;
                }
                self.emit(Opcode::Hash, vec![entry_num]);
            }
            Expression::FnLiteral {
                token: _,
                parameters,
                body,
            } => {
                self.enter_scope();

                for param in parameters {
                    self.symbol_table.define(&param);
                }

                self.compile_program(body.statements)?;
                if self.last_instruction_is(Opcode::Pop) {
                    self.replace_last_pop_with_return();
                }

                if !self.last_instruction_is(Opcode::ReturnValue) {
                    self.emit(Opcode::Return, vec![]);
                }

                let num_locals = self.symbol_table.num_definitions;
                let instructions = self.leave_scope().unwrap();

                let compiled_function = Object::CompiledFunction {
                    instructions,
                    num_locals,
                };
                let constant_index = self.add_constant(compiled_function) as u16;
                self.emit(Opcode::Constant, vec![constant_index]);
            }
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
            Expression::Index {
                token: _,
                left,
                index,
            } => {
                self.compile_expression(*left)?;
                self.compile_expression(*index)?;
                self.emit(Opcode::Index, vec![]);
            }
            Expression::If {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                self.compile_expression(*condition)?;
                // 0xFFFF placeholder value, will be replaced later on
                let jump_cond_pos = self.emit(Opcode::JumpNotTruthy, vec![0xFFFF]);

                self.compile_program(consequence.statements)?;
                if self.last_instruction_is(Opcode::Pop) {
                    self.remove_last_pop();
                }

                // insert a jump instruction after the body if the if statement (to jump over alternative)
                let jump_pos = self.emit(Opcode::Jump, vec![0xFFFF]);

                // Replace conditional jump address with the address of the instruction after the last jump
                let after_conseq_pos = self.current_instructions().len();
                self.change_operand(jump_cond_pos, after_conseq_pos as u16)?;

                if alternative.is_none() {
                    // If statement expression without alternative evaluates to null
                    self.emit(Opcode::Null, vec![]);
                } else {
                    self.compile_program(alternative.unwrap().statements)?;
                    if self.last_instruction_is(Opcode::Pop) {
                        self.remove_last_pop();
                    }
                }

                // Change the jump address after the body to after the alternative
                let after_alternative_pos = self.current_instructions().len();
                self.change_operand(jump_pos, after_alternative_pos as u16)?;
            }
            Expression::Call {
                token: _,
                function,
                arguments,
            } => {
                self.compile_expression(*function)?;
                let num_args = arguments.len();
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                self.emit(Opcode::Call, vec![num_args as u16]);
            }
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

    fn emit(&mut self, opcode: Opcode, operands: Vec<u16>) -> usize {
        let position = self.add_instruction(&make(opcode, operands));
        self.set_last_instruction(opcode, position);
        position
    }

    // do not clear the symbol table and constants for multiple passes in the REPL
    fn clear(&mut self) {
        self.scopes.clear();
        let scope = CompilationScope::default();
        self.scopes.push(scope);
        self.scope_index = 0;
    }

    fn add_symbol(&mut self, name: &str) -> Symbol {
        self.symbol_table.define(name).clone()
    }

    fn resolve_symbol(&mut self, name: &str) -> Result<Symbol, String> {
        let symbol = self.symbol_table.resolve(name);
        match symbol {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(format!("Undefined symbol {}!", name)),
        }
    }

    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        return self.constants.len() - 1;
    }

    fn current_instructions(&mut self) -> &mut Instructions {
        &mut self.scopes[self.scope_index].instructions
    }

    fn add_instruction(&mut self, instruction: &[u8]) -> usize {
        let pos_new_instruction = self.current_instructions().len();
        self.current_instructions()
            .extend(instruction.iter().clone());
        pos_new_instruction
    }

    fn set_last_instruction(&mut self, opcode: Opcode, position: usize) {
        let previous = self.scopes[self.scope_index].last_instruction.clone();
        let last = EmittedInstruction { opcode, position };

        self.scopes[self.scope_index].previous_instruction = previous;
        self.scopes[self.scope_index].last_instruction = last;
    }

    fn last_instruction_is(&self, opcode: Opcode) -> bool {
        self.scopes[self.scope_index].last_instruction.opcode == opcode
    }

    fn remove_last_pop(&mut self) {
        let last = self.scopes[self.scope_index].last_instruction.clone();
        let previous = self.scopes[self.scope_index].previous_instruction.clone();

        self.current_instructions().truncate(last.position);
        self.scopes[self.scope_index].last_instruction = previous;
    }

    fn replace_instruction(&mut self, position: usize, new_instruction: &[u8]) {
        let end = position + new_instruction.len();
        self.current_instructions()
            .splice(position..end, new_instruction.iter().cloned());
    }

    fn replace_last_pop_with_return(&mut self) {
        let last_position = self.scopes[self.scope_index].last_instruction.position;
        self.replace_instruction(last_position, &make(Opcode::ReturnValue, vec![]));
        self.scopes[self.scope_index].last_instruction.opcode = Opcode::ReturnValue;
    }

    fn change_operand(&mut self, op_pos: usize, operand: u16) -> Result<(), String> {
        let opcode: Opcode = self.current_instructions()[op_pos].try_into()?;
        let new_instruction = make(opcode, vec![operand]);
        self.replace_instruction(op_pos, &new_instruction);
        Ok(())
    }

    fn enter_scope(&mut self) {
        let scope = CompilationScope::default();
        self.scopes.push(scope);
        self.scope_index += 1;
        self.symbol_table = SymbolTable::with_enclosed(Rc::from(self.symbol_table.clone()));
    }

    fn leave_scope(&mut self) -> Option<Instructions> {
        let scope = self.scopes.pop()?;
        self.scope_index -= 1;
        // Please do not look at these lines of code, I'm very ashamed of it
        let outer = self.symbol_table.outer.take().unwrap();
        self.symbol_table = Rc::try_unwrap(outer).unwrap();
        Some(scope.instructions)
    }
}
