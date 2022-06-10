use super::object::{Inspectable, Object};

pub type BuiltinFunction = fn(Vec<Object>) -> Result<Object, String>;

pub fn get_builtin(name: &str) -> Option<BuiltinFunction> {
    match name {
        "len" => Some(self::len),
        _ => None,
    }
}

fn len(args: Vec<Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    let arg = &args[0];
    if let Object::String(string) = arg {
        Ok(Object::Integer(string.len().try_into().unwrap()))
    } else {
        Err(format!(
            "argument to len not supported, got={}",
            arg.type_str()
        ))
    }
}
