pub mod object;
mod test;

use self::object::Object;
use crate::parser::ast::{BlockStatement, Expression, Node, Statement};

pub fn eval(node: Node) -> Object {
    match node {
        Node::Statement(statement) => eval_statement(statement),
        Node::Expression(expression) => eval_expression(expression),
        Node::BlockStatement(block_statement) => eval_statements(block_statement.statements),
        Node::Program(program) => eval_statements(program),
    }
}

fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut object = Object::Null;
    for statement in statements {
        object = eval_statement(statement);
        if let Object::ReturnValue(value) = object {
            return *value;
        }
    }
    object
}

fn eval_statement(statement: Statement) -> Object {
    match statement {
        Statement::LetStmt {
            token: _,
            identifier,
            value,
        } => todo!(),
        Statement::ReturnStmt { token: _, value } => {
            Object::ReturnValue(Box::from(eval(Node::Expression(value))))
        }

        Statement::ExpressionStmt {
            token: _,
            expression,
        } => eval_expression(expression),
    }
}

fn eval_expression(expression: Expression) -> Object {
    match expression {
        Expression::IdentifierExpr { token: _, value } => todo!(),
        Expression::LiteralIntExpr { token: _, value } => Object::Integer(value),
        Expression::LiteralBoolExpr { token: _, value } => Object::Boolean(value),
        Expression::LiteralFnExpr {
            token: _,
            parameters,
            body,
        } => todo!(),
        Expression::PrefixExpr {
            token: _,
            operator,
            right_expression,
        } => eval_prefix_expression(&operator, eval(Node::Expression(*right_expression))),
        Expression::InfixExpr {
            token: _,
            left_expression,
            operator,
            right_expression,
        } => eval_infix_expression(
            eval(Node::Expression(*left_expression)),
            &operator,
            eval(Node::Expression(*right_expression)),
        ),
        Expression::IfExpr {
            token: _,
            condition,
            consequence,
            alternative,
        } => eval_if_else_expression(eval(Node::Expression(*condition)), consequence, alternative),
        Expression::CallExpr {
            token: _,
            function,
            arguments,
        } => todo!(),
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Object::Null,
    }
}

fn eval_infix_expression(left: Object, operator: &str, right: Object) -> Object {
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

    // Bool and int are never equal, if the operator is anything else
    // there is no way to produce meaningful values --> return null
    match operator {
        "==" => Object::Boolean(false),
        "!=" => Object::Boolean(true),
        _ => Object::Null,
    }
}

fn eval_integer_infix(left: isize, operator: &str, right: isize) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

fn eval_bool_infix(left: bool, operator: &str, right: bool) -> Object {
    match operator {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    Object::Boolean(!is_truthy(right))
}

fn eval_minus_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(value) => Object::Integer(-value),
        _ => Object::Null,
    }
}

fn eval_if_else_expression(
    condition: Object,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
) -> Object {
    if is_truthy(condition) {
        eval_statements(consequence.statements)
    } else if alternative.is_some() {
        eval_statements(alternative.unwrap().statements)
    } else {
        Object::Null
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
