pub mod object;
mod test;

use self::object::{Inspectable, Object};
use crate::parser::ast::{BlockStatement, Expression, Node, Program, Statement};

pub fn eval(node: Node) -> Result<Object, String> {
    match node {
        Node::Statement(statement) => eval_statement(statement),
        Node::Expression(expression) => eval_expression(expression),
        Node::BlockStatement(block_statement) => eval_block_statement(block_statement),
        Node::Program(program) => eval_program(program),
    }
}

fn eval_program(statements: Program) -> Result<Object, String> {
    let mut result = Object::Null;
    for statement in statements {
        result = eval_statement(statement)?;

        if let Object::ReturnValue(value) = result {
            return Ok(*value);
        }
    }
    Ok(result)
}

fn eval_block_statement(block: BlockStatement) -> Result<Object, String> {
    let mut result = Object::Null;
    for statement in block.statements {
        result = eval_statement(statement)?;
        if let Object::ReturnValue(_) = result {
            return Ok(result);
        }
    }
    Ok(result)
}

fn eval_statement(statement: Statement) -> Result<Object, String> {
    match statement {
        Statement::LetStmt {
            token: _,
            identifier,
            value,
        } => todo!(),
        Statement::ReturnStmt { token: _, value } => {
            let return_object = Box::from(eval(Node::Expression(value))?);
            Ok(Object::ReturnValue(return_object))
        }
        Statement::ExpressionStmt {
            token: _,
            expression,
        } => eval_expression(expression),
    }
}

fn eval_expression(expression: Expression) -> Result<Object, String> {
    match expression {
        Expression::IdentifierExpr { token: _, value } => todo!(),
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
        } => eval_prefix_expression(&operator, eval(Node::Expression(*right_expression))?),
        Expression::InfixExpr {
            token: _,
            left_expression,
            operator,
            right_expression,
        } => eval_infix_expression(
            eval(Node::Expression(*left_expression))?,
            &operator,
            eval(Node::Expression(*right_expression))?,
        ),
        Expression::IfExpr {
            token: _,
            condition,
            consequence,
            alternative,
        } => eval_if_else_expression(
            eval(Node::Expression(*condition))?,
            consequence,
            alternative,
        ),
        Expression::CallExpr {
            token: _,
            function,
            arguments,
        } => todo!(),
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
) -> Result<Object, String> {
    if is_truthy(condition) {
        eval_block_statement(consequence)
    } else if alternative.is_some() {
        eval_block_statement(alternative.unwrap())
    } else {
        Ok(Object::Null)
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
