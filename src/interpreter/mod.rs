mod object;
pub mod test;

use self::object::Object;
use crate::parser::ast::{Expression, Node, Statement};

pub fn eval(node: Node) -> Object {
    match node {
        Node::Statement(_) => todo!(),
        Node::Expression(expression) => eval_expression(expression),
        Node::BlockStatement(_) => todo!(),
        Node::Program(program) => eval_statements(program),
    }
}

fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut object = Object::Null;
    for statement in statements {
        object = eval_statement(statement);
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
        Statement::ReturnStmt { token: _, value } => todo!(),
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
        Expression::LiteralBoolExpr { token: _, value } => todo!(),
        Expression::LiteralFnExpr {
            token: _,
            parameters,
            body,
        } => todo!(),
        Expression::PrefixExpr {
            token: _,
            operator,
            right_expression,
        } => todo!(),
        Expression::InfixExpr {
            token: _,
            left_expression,
            operator,
            right_expression,
        } => todo!(),
        Expression::IfExpr {
            token: _,
            condition,
            consequence,
            alternative,
        } => todo!(),
        Expression::CallExpr {
            token: _,
            function,
            arguments,
        } => todo!(),
    }
}
