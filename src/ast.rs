use crate::token::Token;

pub type Program = Vec<Statement>;

pub enum Statement {
    LetStmt {
        token: Token,
        identifier: String,
        value: Expression,
    },
    ReturnStmt,
}

pub enum Expression {
    IdentifierExpr,
}
