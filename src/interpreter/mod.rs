pub mod environment;
pub mod object;
mod test;

use self::{
    environment::Environment,
    object::{Inspectable, Object},
};
use crate::parser::ast::{BlockStatement, Expression, Node, Program, Statement};

pub fn eval(node: Node, env: &mut Environment) -> Result<Object, String> {
    match node {
        Node::Statement(statement) => eval_statement(statement, env),
        Node::Expression(expression) => eval_expression(expression, env),
        Node::BlockStatement(block_statement) => eval_block_statement(block_statement, env),
        Node::Program(program) => eval_program(program, env),
    }
}

fn eval_program(statements: Program, env: &mut Environment) -> Result<Object, String> {
    let mut result = Object::Null;
    for statement in statements {
        result = eval_statement(statement, env)?;

        if let Object::ReturnValue(value) = result {
            return Ok(*value);
        }
    }
    Ok(result)
}

fn eval_block_statement(block: BlockStatement, env: &mut Environment) -> Result<Object, String> {
    let mut result = Object::Null;
    for statement in block.statements {
        result = eval_statement(statement, env)?;
        if let Object::ReturnValue(_) = result {
            return Ok(result);
        }
    }
    Ok(result)
}

fn eval_statement(statement: Statement, env: &mut Environment) -> Result<Object, String> {
    match statement {
        Statement::LetStmt {
            token: _,
            identifier,
            value,
        } => {
            let val = eval(Node::Expression(value), env)?;
            env.set(&identifier, val);
            Ok(Object::Null)
        }
        Statement::ReturnStmt { token: _, value } => {
            let return_object = Box::from(eval(Node::Expression(value), env)?);
            Ok(Object::ReturnValue(return_object))
        }
        Statement::ExpressionStmt {
            token: _,
            expression,
        } => eval_expression(expression, env),
    }
}

fn eval_expression(expression: Expression, env: &mut Environment) -> Result<Object, String> {
    match expression {
        Expression::IdentifierExpr { token: _, value } => eval_identifier(&value, env),
        Expression::LiteralIntExpr { token: _, value } => Ok(Object::Integer(value)),
        Expression::LiteralBoolExpr { token: _, value } => Ok(Object::Boolean(value)),
        Expression::LiteralFnExpr {
            token: _,
            parameters,
            body,
        } => todo!(),
        Expression::PrefixExpr {
            token: _,
            operator,
            right_expression,
        } => eval_prefix_expression(
            &operator,
            eval(Node::Expression(*right_expression), env)?,
            env,
        ),
        Expression::InfixExpr {
            token: _,
            left_expression,
            operator,
            right_expression,
        } => eval_infix_expression(
            eval(Node::Expression(*left_expression), env)?,
            &operator,
            eval(Node::Expression(*right_expression), env)?,
            env,
        ),
        Expression::IfExpr {
            token: _,
            condition,
            consequence,
            alternative,
        } => eval_if_else_expression(
            eval(Node::Expression(*condition), env)?,
            consequence,
            alternative,
            env,
        ),
        Expression::CallExpr {
            token: _,
            function,
            arguments,
        } => todo!(),
    }
}

fn eval_prefix_expression(
    operator: &str,
    right: Object,
    env: &mut Environment,
) -> Result<Object, String> {
    match operator {
        "!" => eval_bang_operator_expression(right, env),
        "-" => eval_minus_operator_expression(right, env),
        _ => Err(format!(
            "unknown operator: {}{}",
            operator,
            right.type_str()
        )),
    }
}

fn eval_infix_expression(
    left: Object,
    operator: &str,
    right: Object,
    env: &mut Environment,
) -> Result<Object, String> {
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
            return eval_integer_infix(left_value, operator, right_value, env);
        }
    }

    if let Object::Boolean(left_value) = left {
        if let Object::Boolean(right_value) = right {
            return eval_bool_infix(left_value, operator, right_value, env);
        }
    }

    Err(format!(
        "unknown operator: {} {} {}",
        left.type_str(),
        operator,
        right.type_str()
    ))
}

fn eval_integer_infix(
    left: isize,
    operator: &str,
    right: isize,
    env: &mut Environment,
) -> Result<Object, String> {
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

fn eval_bool_infix(
    left: bool,
    operator: &str,
    right: bool,
    env: &mut Environment,
) -> Result<Object, String> {
    match operator {
        "==" => Ok(Object::Boolean(left == right)),
        "!=" => Ok(Object::Boolean(left != right)),
        _ => Err(format!("unknown operator: BOOLEAN {} BOOLEAN", operator,)),
    }
}

fn eval_bang_operator_expression(right: Object, env: &mut Environment) -> Result<Object, String> {
    Ok(Object::Boolean(!is_truthy(right)))
}

fn eval_minus_operator_expression(right: Object, env: &mut Environment) -> Result<Object, String> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(-value)),
        _ => Err(format!("unknown operator: -{}", right.type_str())),
    }
}

fn eval_if_else_expression(
    condition: Object,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
    env: &mut Environment,
) -> Result<Object, String> {
    if is_truthy(condition) {
        eval_block_statement(consequence, env)
    } else if alternative.is_some() {
        eval_block_statement(alternative.unwrap(), env)
    } else {
        Ok(Object::Null)
    }
}

fn eval_identifier(name: &str, env: &mut Environment) -> Result<Object, String> {
    let value = env.get(name);
    match value {
        Some(object) => Ok(object.clone()),
        None => Err(format!("unknown identifier: {}", name)),
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Integer(value) => value != 0,
        Object::Boolean(value) => value,
        Object::ReturnValue(value) => is_truthy(*value),
        Object::Null => false,
    }
}
