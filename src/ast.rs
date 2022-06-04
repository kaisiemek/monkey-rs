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
}

#[derive(Debug)]
pub enum Expression {
    IdentifierExpr,
}
