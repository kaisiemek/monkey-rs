use std::collections::HashMap;

use crate::parser::ast::{BlockStatement, Expression, Program, Statement};

pub type ModifierFunc = fn(&Expression) -> Expression;

pub fn modify_block_statement(mut block: BlockStatement, modifier: ModifierFunc) -> BlockStatement {
    block.statements = modify_statements(block.statements, modifier);
    return block;
}

pub fn modify_statement(statement: Statement, modifier: ModifierFunc) -> Statement {
    match statement {
        Statement::Let {
            token,
            identifier,
            value,
        } => Statement::Let {
            token,
            identifier,
            value: modify_expression(value, modifier),
        },
        Statement::Return { token, value } => Statement::Return {
            token,
            value: modify_expression(value, modifier),
        },
        Statement::Expression { token, expression } => Statement::Expression {
            token,
            expression: modify_expression(expression, modifier),
        },
    }
}

pub fn modify_expression(expression: Expression, modifier: ModifierFunc) -> Expression {
    match expression {
        Expression::Infix {
            token,
            left,
            operator,
            right,
        } => Expression::Infix {
            token,
            left: Box::from(modify_expression(*left, modifier)),
            operator,
            right: Box::from(modify_expression(*right, modifier)),
        },
        Expression::Prefix {
            token,
            operator,
            right,
        } => Expression::Prefix {
            token,
            operator,
            right: Box::from(modify_expression(*right, modifier)),
        },
        Expression::Index { token, left, index } => Expression::Index {
            token,
            left: Box::from(modify_expression(*left, modifier)),
            index: Box::from(modify_expression(*index, modifier)),
        },
        Expression::If {
            token,
            condition,
            consequence,
            alternative,
        } => {
            let new_alternative;
            if alternative.is_some() {
                new_alternative = Some(modify_block_statement(alternative.unwrap(), modifier));
            } else {
                new_alternative = None
            }

            Expression::If {
                token,
                condition: Box::from(modify_expression(*condition, modifier)),
                consequence: modify_block_statement(consequence, modifier),
                alternative: new_alternative,
            }
        }
        Expression::FnLiteral {
            token,
            mut parameters,
            body,
        } => {
            for expr in &mut parameters {
                *expr = modify_expression(expr.to_owned(), modifier);
            }

            Expression::FnLiteral {
                token,
                parameters,
                body: modify_block_statement(body, modifier),
            }
        }
        Expression::ArrayLiteral {
            token,
            mut elements,
        } => {
            for expr in &mut elements {
                *expr = modify_expression(expr.to_owned(), modifier);
            }

            Expression::ArrayLiteral { token, elements }
        }
        Expression::HashLiteral { token, entries } => {
            let mut new_map = HashMap::new();

            for (key, val) in entries {
                new_map.insert(
                    modify_expression(key, modifier),
                    modify_expression(val, modifier),
                );
            }

            Expression::HashLiteral {
                token,
                entries: new_map,
            }
        }
        _ => modifier(&expression),
    }
}

fn modify_statements(mut statements: Vec<Statement>, modifier: ModifierFunc) -> Vec<Statement> {
    for statement in &mut statements {
        *statement = modify_statement(statement.to_owned(), modifier);
    }

    return statements;
}
