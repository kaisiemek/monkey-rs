#[derive(Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
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
        }
    }
}
