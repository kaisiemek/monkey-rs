use std::{cell::RefCell, rc::Rc};

use crate::parser::ast::{BlockStatement, Expression};

use super::environment::Environment;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Function {
        parameters: Vec<Expression>,
        body: BlockStatement,
        environment: Rc<RefCell<Environment>>,
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
        }
    }

    fn type_str(&self) -> String {
        match self {
            Object::Integer(_) => String::from("INTEGER"),
            Object::Boolean(_) => String::from("BOOLEAN"),
            Object::ReturnValue(value) => format!("RETURN {}", value.type_str()),
            Object::Function { .. } => String::from("FUNCTION"),
            Object::Null => String::from("NULL"),
        }
    }
}
