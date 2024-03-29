pub mod builtins;

use crate::{
    code::Instructions, interpreter::environment::Environment, parser::ast::BlockStatement,
};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Hash(HashMap<Object, Object>),
    ReturnValue(Box<Object>),
    InterpretedFunction {
        parameters: Vec<String>,
        body: BlockStatement,
        environment: Rc<RefCell<Environment>>,
    },
    BuiltIn {
        name: String,
    },
    Null,
    CompiledFunction {
        instructions: Instructions,
        num_locals: usize,
        num_parameters: usize,
    },
}

pub trait Inspectable {
    fn inspect(&self) -> String;
    fn type_str(&self) -> String;
}

impl Inspectable for Object {
    fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{}", value),
            Object::Boolean(value) => format!("{}", value),
            Object::String(value) => format!("\"{}\"", value),
            Object::Array(elements) => {
                let expr_strings: Vec<String> = elements.iter().map(|val| val.inspect()).collect();
                format!("[{}]", expr_strings.join(", "))
            }
            Object::Hash(entries) => {
                let entry_strings: Vec<String> = entries
                    .iter()
                    .map(|(key, val)| format!("{}: {}", key.inspect(), val.inspect()))
                    .collect();

                format!("{{{}}}", entry_strings.join(", "))
            }
            Object::ReturnValue(value) => format!("{}", value.inspect()),
            Object::InterpretedFunction {
                parameters,
                body,
                environment: _,
            } => {
                let param_strings: Vec<String> =
                    parameters.iter().map(|param| param.to_string()).collect();
                format!("fn({}) {}", param_strings.join(", "), body.to_string())
            }
            Object::Null => String::from("null"),
            Object::BuiltIn { name, .. } => format!("Built-in function {}", name),

            Object::CompiledFunction {
                instructions,
                num_locals,
                num_parameters,
            } => {
                format!(
                    "Compiled function ({} parameters, {} locals): {:?}",
                    num_parameters, num_locals, instructions
                )
            }
        }
    }

    fn type_str(&self) -> String {
        match self {
            Object::Integer(_) => String::from("INTEGER"),
            Object::Boolean(_) => String::from("BOOLEAN"),
            Object::String(_) => String::from("STRING"),
            Object::Array(_) => String::from("ARRAY"),
            Object::Hash(_) => String::from("HASH"),
            Object::ReturnValue(value) => format!("RETURN {}", value.type_str()),
            Object::InterpretedFunction { .. } => String::from("FUNCTION"),
            Object::BuiltIn { .. } => String::from("BUILTIN"),
            Object::Null => String::from("NULL"),
            Object::CompiledFunction { .. } => String::from("COMPILED FUNCTION"),
        }
    }
}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Object::Integer(value) => value.hash(state),
            Object::Boolean(value) => value.hash(state),
            Object::String(value) => value.hash(state),
            obj => panic!("Can't hash {}", obj.type_str()),
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Null
    }
}

impl Default for &Object {
    fn default() -> Self {
        &Object::Null
    }
}
