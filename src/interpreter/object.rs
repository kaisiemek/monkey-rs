#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
}

pub trait Inspectable {
    fn inspect(&self) -> String;
}

impl Inspectable for Object {
    fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{}", value),
            Object::Boolean(value) => format!("{}", value),
            Object::Null => String::from("null"),
            Object::ReturnValue(value) => format!("{}", value.inspect()),
        }
    }
}
