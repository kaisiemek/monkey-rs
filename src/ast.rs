use crate::token::Token;

trait Node {
    fn token_literal(&self) -> &str;
}

pub enum Statement {
    LetStmt {
        token: Token,
        name: IdentifierExpr,
        value: Expression,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::LetStmt { token, .. } => token.literal.as_str(),
        }
    }
}

struct IdentifierExpr {
    token: Token,
    value: String,
}

enum Expression {
    IdentifierExpr(IdentifierExpr),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}
