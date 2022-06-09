use crate::lexer::token::Token;

pub type Program = Vec<Statement>;

pub enum Node {
    Statement(Statement),
    Expression(Expression),
    BlockStatement(BlockStatement),
    Program(Program),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::LetStmt {
                token,
                identifier,
                value,
            } => format!("{} {} = {}", token.literal, identifier, value.to_string()),
            Statement::ReturnStmt { token, value } => {
                format!("{} {}", token.literal, value.to_string())
            }
            Statement::ExpressionStmt {
                token: _,
                expression,
            } => format!("{}", expression.to_string()),
        }
    }
}

impl ToString for BlockStatement {
    fn to_string(&self) -> String {
        let mut stmt_strings = Vec::new();

        for statement in &self.statements {
            stmt_strings.push(statement.to_string());
        }

        format!("{{\n\t{}\n}}", stmt_strings.join("\n\t"))
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    IdentifierExpr {
        token: Token,
        value: String,
    },
    LiteralIntExpr {
        token: Token,
        value: isize,
    },
    LiteralBoolExpr {
        token: Token,
        value: bool,
    },
    LiteralFnExpr {
        token: Token,
        parameters: Vec<Expression>,
        body: BlockStatement,
    },
    PrefixExpr {
        token: Token,
        operator: String,
        right_expression: Box<Expression>,
    },
    InfixExpr {
        token: Token,
        left_expression: Box<Expression>,
        operator: String,
        right_expression: Box<Expression>,
    },
    IfExpr {
        token: Token,
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    CallExpr {
        token: Token,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::IdentifierExpr { token: _, value } => String::from(value),
            Expression::LiteralIntExpr { token: _, value } => format!("{}", value),
            Expression::LiteralBoolExpr { token: _, value } => format!("{}", value),
            Expression::PrefixExpr {
                token: _,
                operator,
                right_expression,
            } => format!("({}{})", operator, right_expression.to_string()),
            Expression::InfixExpr {
                token: _,
                left_expression,
                operator,
                right_expression,
            } => format!(
                "({} {} {})",
                left_expression.to_string(),
                operator,
                right_expression.to_string()
            ),
            Expression::IfExpr {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                let mut blockstring =
                    format!("if {} {}", condition.to_string(), consequence.to_string());

                if alternative.is_some() {
                    blockstring.push_str(
                        format!(" else {}", alternative.as_ref().unwrap().to_string()).as_str(),
                    );
                }
                blockstring
            }
            Expression::LiteralFnExpr {
                token: _,
                parameters,
                body,
            } => {
                let mut param_strings: Vec<String> = vec![];
                for parameter in parameters {
                    param_strings.push(parameter.to_string());
                }

                format!("fn({}) {}", param_strings.join(", "), body.to_string())
            }
            Expression::CallExpr {
                token: _,
                function,
                arguments,
            } => {
                let mut arg_strings: Vec<String> = vec![];
                for argument in arguments {
                    arg_strings.push(argument.to_string());
                }

                format!("{}({})", function.to_string(), arg_strings.join(", "))
            }
        }
    }
}
