mod builtins;
pub mod environment;
pub mod object;
mod test;

use std::{cell::RefCell, rc::Rc};

use self::{
    builtins::get_builtin,
    environment::Environment,
    object::{Inspectable, Object},
};
use crate::parser::ast::{BlockStatement, Expression, Program, Statement};

pub fn eval_program(statements: Program, env: Rc<RefCell<Environment>>) -> Result<Object, String> {
    let mut result = Object::Null;
    for statement in statements {
        result = eval_statement(statement, env.clone())?;

        if let Object::ReturnValue(value) = result {
            return Ok(*value);
        }
    }
    Ok(result)
}

fn eval_block_statement(
    block: BlockStatement,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    let mut result = Object::Null;
    for statement in block.statements {
        result = eval_statement(statement, env.clone())?;
        if let Object::ReturnValue(_) = result {
            return Ok(result);
        }
    }
    Ok(result)
}

fn eval_statement(statement: Statement, env: Rc<RefCell<Environment>>) -> Result<Object, String> {
    match statement {
        Statement::LetStmt {
            token: _,
            identifier,
            value,
        } => {
            let val = eval_expression(value, env.clone())?;
            (*env).borrow_mut().set(&identifier, val);
            Ok(Object::Null)
        }
        Statement::ReturnStmt { token: _, value } => {
            let return_object = Box::from(eval_expression(value, env)?);
            Ok(Object::ReturnValue(return_object))
        }
        Statement::ExpressionStmt {
            token: _,
            expression,
        } => eval_expression(expression, env),
    }
}

fn eval_expression(
    expression: Expression,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    match expression {
        Expression::IdentifierExpr { token: _, value } => eval_identifier(&value, env),
        Expression::LiteralIntExpr { token: _, value } => Ok(Object::Integer(value)),
        Expression::LiteralBoolExpr { token: _, value } => Ok(Object::Boolean(value)),
        Expression::LiteralStringExpr { token: _, value } => Ok(Object::String(value)),
        Expression::LiteralArrayExpr { token: _, elements } => eval_array(elements, env),
        Expression::LiteralHashExpr { token, entries } => todo!(),
        Expression::LiteralFnExpr {
            token: _,
            parameters,
            body,
        } => Ok(Object::Function {
            parameters,
            body,
            environment: env,
        }),
        Expression::PrefixExpr {
            token: _,
            operator,
            right_expression,
        } => eval_prefix_expression(&operator, eval_expression(*right_expression, env)?),
        Expression::InfixExpr {
            token: _,
            left_expression,
            operator,
            right_expression,
        } => eval_infix_expression(
            eval_expression(*left_expression, env.clone())?,
            &operator,
            eval_expression(*right_expression, env.clone())?,
        ),
        Expression::IndexExpr {
            token: _,
            left,
            index,
        } => eval_index_expression(
            eval_expression(*left, env.clone())?,
            eval_expression(*index, env.clone())?,
        ),
        Expression::IfExpr {
            token: _,
            condition,
            consequence,
            alternative,
        } => eval_if_else_expression(
            eval_expression(*condition, env.clone())?,
            consequence,
            alternative,
            env,
        ),
        Expression::CallExpr {
            token: _,
            function,
            arguments,
        } => eval_call_expression(function, arguments, env),
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Result<Object, String> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Err(format!(
            "unknown operator: {}{}",
            operator,
            right.type_str()
        )),
    }
}

fn eval_infix_expression(left: Object, operator: &str, right: Object) -> Result<Object, String> {
    if left.type_str() != right.type_str() {
        return Err(format!(
            "type mismatch: {} {} {}",
            left.type_str(),
            operator,
            right.type_str()
        ));
    }

    if let Object::Integer(left_value) = left {
        if let Object::Integer(right_value) = right {
            return eval_integer_infix(left_value, operator, right_value);
        }
    }

    if let Object::Boolean(left_value) = left {
        if let Object::Boolean(right_value) = right {
            return eval_bool_infix(left_value, operator, right_value);
        }
    }

    if let Object::String(left_value) = &left {
        if let Object::String(right_value) = &right {
            return eval_string_infix(left_value, operator, right_value);
        }
    }

    Err(format!(
        "unknown operator: {} {} {}",
        left.type_str(),
        operator,
        right.type_str()
    ))
}

fn eval_integer_infix(left: isize, operator: &str, right: isize) -> Result<Object, String> {
    match operator {
        "+" => Ok(Object::Integer(left + right)),
        "-" => Ok(Object::Integer(left - right)),
        "*" => Ok(Object::Integer(left * right)),
        "/" => Ok(Object::Integer(left / right)),
        "<" => Ok(Object::Boolean(left < right)),
        ">" => Ok(Object::Boolean(left > right)),
        "==" => Ok(Object::Boolean(left == right)),
        "!=" => Ok(Object::Boolean(left != right)),
        _ => Err(format!("unknown operator: INTEGER {} INTEGER", operator,)),
    }
}

fn eval_bool_infix(left: bool, operator: &str, right: bool) -> Result<Object, String> {
    match operator {
        "==" => Ok(Object::Boolean(left == right)),
        "!=" => Ok(Object::Boolean(left != right)),
        _ => Err(format!("unknown operator: BOOLEAN {} BOOLEAN", operator,)),
    }
}

fn eval_string_infix(left: &str, operator: &str, right: &str) -> Result<Object, String> {
    match operator {
        "+" => Ok(Object::String(format!("{}{}", left, right))),
        _ => Err(format!("unknown operator: STRING {} STRING", operator)),
    }
}

fn eval_bang_operator_expression(right: Object) -> Result<Object, String> {
    Ok(Object::Boolean(!is_truthy(right)))
}

fn eval_minus_operator_expression(right: Object) -> Result<Object, String> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(-value)),
        _ => Err(format!("unknown operator: -{}", right.type_str())),
    }
}

fn eval_if_else_expression(
    condition: Object,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    if is_truthy(condition) {
        eval_block_statement(consequence, env)
    } else if alternative.is_some() {
        eval_block_statement(alternative.unwrap(), env)
    } else {
        Ok(Object::Null)
    }
}

fn eval_call_expression(
    func: Box<Expression>,
    arguments: Vec<Expression>,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    let function = eval_expression(*func, env.clone())?;
    let mut args = Vec::new();

    for expression in arguments {
        args.push(eval_expression(expression, env.clone())?);
    }

    apply_function(function, args)
}

fn eval_identifier(name: &str, env: Rc<RefCell<Environment>>) -> Result<Object, String> {
    let environment = env.borrow();
    let value = environment.get(name);
    if value.is_some() {
        return Ok(value.unwrap().clone());
    }

    let builtin = get_builtin(name);

    match builtin {
        Some(function) => Ok(Object::BuiltIn {
            name: name.to_string(),
            function,
        }),
        None => Err(format!("unknown identifier: {}", name)),
    }
}

fn eval_array(
    element_expressions: Vec<Expression>,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    let mut elements = Vec::new();

    for expression in element_expressions {
        elements.push(eval_expression(expression, env.clone())?);
    }

    Ok(Object::Array(elements))
}

fn eval_index_expression(left: Object, index: Object) -> Result<Object, String> {
    if let Object::Integer(i) = index {
        match left {
            Object::Array(elements) => Ok(eval_array_index_expression(&elements, i)),
            other => Err(format!(
                "index operator not supported: {}",
                other.type_str()
            )),
        }
    } else {
        Err(format!(
            "index must be an integer, got: {}",
            index.type_str()
        ))
    }
}

fn eval_array_index_expression(elements: &Vec<Object>, index: isize) -> Object {
    if index < 0 {
        return Object::Null;
    };
    let result = elements.get(index as usize);
    result.unwrap_or(&Object::Null).clone()
}

fn apply_function(function: Object, arguments: Vec<Object>) -> Result<Object, String> {
    match function {
        Object::Function {
            parameters,
            body,
            environment,
        } => {
            let extended_env = extend_function_env(&parameters, &arguments, environment)?;
            eval_block_statement(body, extended_env)
        }
        Object::BuiltIn { name: _, function } => function(arguments),
        other => Err(format!("not a function: {}", other.type_str())),
    }
}

fn extend_function_env(
    parameters: &[Expression],
    arguments: &[Object],
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<RefCell<Environment>>, String> {
    let mut extended_env = Environment::new_enclosed(env);
    if parameters.len() != arguments.len() {
        return Err(format!(
            "the amount of supplied arguments {} differed from the specified parameters {}",
            arguments.len(),
            parameters.len(),
        ));
    }

    for (param, arg) in parameters.iter().zip(arguments.iter()) {
        if let Expression::IdentifierExpr {
            token: _,
            value: name,
        } = param
        {
            extended_env.set(name, arg.clone());
        } else {
            return Err(format!(
                "Expected all parameters to be IdentifierExpr, but got {:?}",
                param
            ));
        }
    }

    Ok(Rc::new(RefCell::new(extended_env)))
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Integer(value) => value != 0,
        Object::Boolean(value) => value,
        Object::Null => false,
        Object::ReturnValue(value) => is_truthy(*value),
        _ => true,
    }
}
