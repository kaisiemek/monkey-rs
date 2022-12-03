use crate::lexer::token::Token;
use std::{collections::HashMap, hash::Hash};

pub type Program = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let {
        token: Token,
        identifier: String,
        value: Expression,
    },
    Return {
        token: Token,
        value: Expression,
    },
    Expression {
        token: Token,
        expression: Expression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier {
        token: Token,
        value: String,
    },
    IntLiteral {
        token: Token,
        value: isize,
    },
    BoolLiteral {
        token: Token,
        value: bool,
    },
    StringLiteral {
        token: Token,
        value: String,
    },
    ArrayLiteral {
        token: Token,
        elements: Vec<Expression>,
    },
    HashLiteral {
        token: Token,
        entries: HashMap<Expression, Expression>,
    },
    FnLiteral {
        token: Token,
        parameters: Vec<String>,
        body: BlockStatement,
    },
    Prefix {
        token: Token,
        operator: String,
        right_expression: Box<Expression>,
    },
    Infix {
        token: Token,
        left_expression: Box<Expression>,
        operator: String,
        right_expression: Box<Expression>,
    },
    Index {
        token: Token,
        left: Box<Expression>,
        index: Box<Expression>,
    },
    If {
        token: Token,
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    Call {
        token: Token,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let {
                token,
                identifier,
                value,
            } => format!("{} {} = {}", token.literal, identifier, value.to_string()),
            Statement::Return { token, value } => {
                format!("{} {}", token.literal, value.to_string())
            }
            Statement::Expression {
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

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier { token: _, value } => String::from(value),
            Expression::IntLiteral { token: _, value } => format!("{}", value),
            Expression::BoolLiteral { token: _, value } => format!("{}", value),
            Expression::StringLiteral { token: _, value } => value.clone(),
            Expression::ArrayLiteral {
                token: _,
                elements: value,
            } => {
                let expr_strings: Vec<String> = value.iter().map(|val| val.to_string()).collect();
                format!("[{}]", expr_strings.join(", "))
            }
            Expression::HashLiteral { token: _, entries } => {
                let entry_strings: Vec<String> = entries
                    .iter()
                    .map(|(key, val)| format!("{}: {}", key.to_string(), val.to_string()))
                    .collect();

                format!("{{{}}}", entry_strings.join(", "))
            }
            Expression::Prefix {
                token: _,
                operator,
                right_expression,
            } => format!("({}{})", operator, right_expression.to_string()),
            Expression::Infix {
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
            Expression::Index {
                token: _,
                left,
                index,
            } => {
                format!("({}[{}])", left.to_string(), index.to_string())
            }
            Expression::If {
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
            Expression::FnLiteral {
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
            Expression::Call {
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

impl Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let string_representation = self.to_string();
        string_representation.hash(state);
        core::mem::discriminant(self).hash(state);
    }
}
