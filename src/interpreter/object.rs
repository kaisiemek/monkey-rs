#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    ReturnValue(Box<Object>),
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
            Object::Null => String::from("null"),
        }
    }

    fn type_str(&self) -> String {
        match self {
            Object::Integer(_) => String::from("INTEGER"),
            Object::Boolean(_) => String::from("BOOLEAN"),
            Object::ReturnValue(value) => format!("RETURN {}", value.type_str()),
            Object::Null => String::from("NULL"),
        }
    }
}
