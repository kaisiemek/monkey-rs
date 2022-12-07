use super::{Inspectable, Object};

pub const BUILTIN_NAMES: [&str; 7] = ["len", "print", "first", "last", "tail", "push", "typeof"];

pub struct BuiltinFunction<'a> {
    pub name: String,
    pub func: &'a dyn Fn(Vec<Object>) -> Result<Object, String>,
}

pub fn get_builtin(name: &str) -> Option<BuiltinFunction> {
    let builtins: Vec<BuiltinFunction> = vec![
        BuiltinFunction {
            name: "len".to_string(),
            func: &self::len,
        },
        BuiltinFunction {
            name: "print".to_string(),
            func: &self::print,
        },
        BuiltinFunction {
            name: "first".to_string(),
            func: &self::first,
        },
        BuiltinFunction {
            name: "last".to_string(),
            func: &self::last,
        },
        BuiltinFunction {
            name: "tail".to_string(),
            func: &self::tail,
        },
        BuiltinFunction {
            name: "push".to_string(),
            func: &self::push,
        },
        BuiltinFunction {
            name: "typeof".to_string(),
            func: &self::type_of,
        },
    ];

    for builtin in builtins {
        if builtin.name == name {
            return Some(builtin);
        }
    }

    None
}

fn len(args: Vec<Object>) -> Result<Object, String> {
    assert_arg_number(&args, 1)?;

    match &args[0] {
        Object::String(string) => Ok(Object::Integer(string.len().try_into().unwrap_or_default())),
        Object::Array(values) => Ok(Object::Integer(values.len().try_into().unwrap_or_default())),
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

fn first(args: Vec<Object>) -> Result<Object, String> {
    assert_arg_number(&args, 1)?;

    if let Object::Array(elements) = &args[0] {
        Ok(elements.first().unwrap_or(&Object::Null).clone())
    } else {
        Err(format!(
            "argument to first must be ARRAY, got={}",
            &args[0].type_str()
        ))
    }
}

fn last(args: Vec<Object>) -> Result<Object, String> {
    assert_arg_number(&args, 1)?;

    if let Object::Array(elements) = &args[0] {
        Ok(elements.last().unwrap_or(&Object::Null).clone())
    } else {
        Err(format!(
            "argument to last must be ARRAY, got={}",
            &args[0].type_str()
        ))
    }
}

fn tail(args: Vec<Object>) -> Result<Object, String> {
    assert_arg_number(&args, 1)?;

    if let Object::Array(elements) = &args[0] {
        if elements.len() < 1 {
            return Ok(Object::Null);
        }

        let tail_elements: Vec<Object> = elements.iter().skip(1).map(|el| el.clone()).collect();
        Ok(Object::Array(tail_elements))
    } else {
        Err(format!(
            "argument to tail must be ARRAY, got={}",
            &args[0].type_str()
        ))
    }
}

fn push(args: Vec<Object>) -> Result<Object, String> {
    assert_arg_number(&args, 2)?;
    let elements;

    if let Object::Array(els) = &args[0] {
        elements = els;
    } else {
        return Err(format!(
            "first argument to push must be ARRAY, got={}",
            &args[0].type_str()
        ));
    }

    let mut new_elements = elements.clone();
    new_elements.push(args[1].clone());

    Ok(Object::Array(new_elements))
}

fn assert_arg_number(args: &Vec<Object>, expected: usize) -> Result<(), String> {
    if args.len() != expected {
        Err(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            expected
        ))
    } else {
        Ok(())
    }
}
