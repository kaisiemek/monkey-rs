use super::object::{Inspectable, Object};

pub type BuiltinFunction = fn(Vec<Object>) -> Result<Object, String>;

pub fn get_builtin(name: &str) -> Option<BuiltinFunction> {
    match name {
        "len" => Some(self::len),
        "print" => Some(self::print),
        "typeof" => Some(self::type_of),
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

    match &args[0] {
        Object::String(string) => Ok(Object::Integer(string.len().try_into().unwrap())),
        other => Err(format!(
            "argument to len not supported, got={}",
            other.type_str()
        )),
    }
}

fn print(args: Vec<Object>) -> Result<Object, String> {
    for arg in args {
        println!("{}", arg.inspect());
    }

    Ok(Object::Null)
}

fn type_of(args: Vec<Object>) -> Result<Object, String> {
    for arg in args {
        println!("{}", arg.type_str());
    }

    Ok(Object::Null)
}
