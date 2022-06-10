use std::{cell::RefCell, rc::Rc};

use crate::parser::ast::{BlockStatement, Expression};

use super::{builtins::BuiltinFunction, environment::Environment};

#[derive(Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    String(String),
    ReturnValue(Box<Object>),
    Function {
        parameters: Vec<Expression>,
        body: BlockStatement,
        environment: Rc<RefCell<Environment>>,
    },
    BuiltIn {
        name: String,
        function: BuiltinFunction,
    },
    Null,
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
            Object::String(value) => value.clone(),
            Object::ReturnValue(value) => format!("{}", value.inspect()),
            Object::Function {
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
        }
    }

    fn type_str(&self) -> String {
        match self {
            Object::Integer(_) => String::from("INTEGER"),
            Object::Boolean(_) => String::from("BOOLEAN"),
            Object::String(_) => String::from("STRING"),
            Object::ReturnValue(value) => format!("RETURN {}", value.type_str()),
            Object::Function { .. } => String::from("FUNCTION"),
            Object::BuiltIn { .. } => String::from("BUILTIN"),
            Object::Null => String::from("NULL"),
        }
    }
}
