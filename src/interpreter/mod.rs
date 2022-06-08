mod object;
pub mod test;

use self::object::Object;
use crate::parser::ast::Node;

pub fn eval(node: Node) -> Object {
    Object::Null
}
