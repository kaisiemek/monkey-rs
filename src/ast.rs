#![allow(dead_code)]
use crate::token::Token;

pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    LetStmt {
        token: Token,
        identifier: String,
        value: Expression,
    },
    ReturnStmt {
        token: Token,
        value: Expression,
    },
    ExpressionStmt {
        token: Token,
        expression: Expression,
    },
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::LetStmt {
                token,
                identifier,
                value,
            } => format!("{} {} = {};", token.literal, identifier, value.to_string()),
            Statement::ReturnStmt { token, value } => {
                format!("{} {};", token.literal, value.to_string())
            }
            Statement::ExpressionStmt {
                token: _,
                expression,
            } => format!("{};", expression.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    DevExpr,
    IdentifierExpr { token: Token, value: String },
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        String::from("<IMPLEMENT EXPRESSION ToString TRAIT>")
    }
}
