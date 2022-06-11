pub mod ast;
mod precedence;
mod test;

use std::collections::HashMap;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};

use self::{
    ast::{BlockStatement, Expression, Program, Statement},
    precedence::{get_operator_precedence, Precedence},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

/*
    ==================================================
    PUBLIC INTERFACE
    ==================================================
*/
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token {
                tok_type: TokenType::EOF,
                literal: String::from(""),
            },
            peek_token: Token {
                tok_type: TokenType::EOF,
                literal: String::from(""),
            },
        };

        // Initialise parser by setting the cur_ and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<String>> {
        let mut program: Program = Vec::new();
        let mut errors = Vec::new();
        while !self.cur_token_is(TokenType::EOF) {
            match self.parse_statement() {
                Ok(statement) => program.push(statement),
                Err(error) => errors.push(error),
            }
            self.next_token();
        }

        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }
}

/*
    ==================================================
    PARSER TOKEN & ERROR METHODS
    ==================================================
*/
impl Parser {
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn cur_token_is(&self, of_type: TokenType) -> bool {
        return self.cur_token.tok_type == of_type;
    }

    fn peek_token_is(&self, of_type: TokenType) -> bool {
        return self.peek_token.tok_type == of_type;
    }

    fn expect_peek(&mut self, expected_type: TokenType) -> Result<(), String> {
        if !self.peek_token_is(expected_type) {
            Err(format!(
                "Expected a token of type {}, but got {} instead",
                expected_type.to_string(),
                self.peek_token.tok_type.to_string()
            ))
        } else {
            self.next_token();
            Ok(())
        }
    }

    fn peek_precedence(&self) -> Precedence {
        get_operator_precedence(self.peek_token.tok_type)
    }

    fn current_precedence(&self) -> Precedence {
        get_operator_precedence(self.cur_token.tok_type)
    }
}

/*
    ==================================================
    STATEMENT PARSING METHODS
    ==================================================
*/
impl Parser {
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.cur_token.tok_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone();
        self.expect_peek(TokenType::Ident)?;

        let identifier = self.cur_token.literal.clone();

        self.expect_peek(TokenType::Assign)?;
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;

        while !self.cur_token_is(TokenType::Semicolon) && !self.cur_token_is(TokenType::EOF) {
            self.next_token();
        }

        Ok(Statement::Let {
            token,
            identifier,
            value,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone();

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;
        while !self.cur_token_is(TokenType::Semicolon) {
            if self.cur_token_is(TokenType::EOF) {
                return Err("Unexpected EOF in return statement".to_string());
            }
            self.next_token();
        }
        Ok(Statement::Return { token, value })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Expression { token, expression })
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, String> {
        let token = self.cur_token.clone();
        let mut statements: Vec<Statement> = Vec::new();

        self.next_token();
        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }

        Ok(BlockStatement { token, statements })
    }
}

/*
    ==================================================
    EXPRESSION PARSING METHODS
    ==================================================
*/
impl Parser {
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let mut left_expression = self.parse_prefix_expression()?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            left_expression = self.parse_infix_expression(left_expression.clone())?;
        }

        Ok(left_expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        match self.cur_token.tok_type {
            TokenType::Ident => Ok(self.parse_identifier()),
            TokenType::Int | TokenType::String | TokenType::True | TokenType::False => {
                Ok(self.parse_literal()?)
            }
            TokenType::Bang | TokenType::Minus => {
                let token = self.cur_token.clone();
                let operator = self.cur_token.literal.clone();
                self.next_token();
                let right_expression = Box::new(self.parse_expression(Precedence::Prefix)?);
                Ok(Expression::Prefix {
                    token,
                    operator,
                    right_expression,
                })
            }
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_fn_expression(),
            TokenType::LBracket => self.parse_array_literal(),
            TokenType::LBrace => self.parse_hash_literal(),
            other => Err(format!(
                "Couldn't parse prefix expression for {}",
                other.to_string()
            )),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: Expression,
    ) -> Result<Expression, String> {
        match self.peek_token.tok_type {
            TokenType::LParen => self.parse_call_expression(left_expression),
            TokenType::LBracket => self.parse_index_expression(left_expression),
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Asterisk
            | TokenType::Slash
            | TokenType::Eq
            | TokenType::NotEq
            | TokenType::Gt
            | TokenType::Lt => self.parse_infix_inner(left_expression),
            other => Err(format!(
                "No infix expression for {} found",
                other.to_string()
            )),
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RParen)?;
        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();

        self.expect_peek(TokenType::LParen)?;
        self.next_token();

        let condition = Box::new(self.parse_expression(Precedence::Lowest)?);

        self.expect_peek(TokenType::RParen)?;
        self.expect_peek(TokenType::LBrace)?;

        let consequence = self.parse_block_statement()?;

        let mut alternative: Option<BlockStatement> = None;
        if self.peek_token_is(TokenType::Else) {
            self.next_token();
            self.expect_peek(TokenType::LBrace)?;
            alternative = Some(self.parse_block_statement()?);
        }

        Ok(Expression::If {
            token,
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_fn_expression(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();
        self.expect_peek(TokenType::LParen)?;

        let parameters = self.parse_expression_list(TokenType::RParen)?;

        self.expect_peek(TokenType::LBrace)?;

        let body = self.parse_block_statement()?;

        Ok(Expression::FnLiteral {
            token,
            parameters,
            body,
        })
    }

    fn parse_call_expression(&mut self, left_expression: Expression) -> Result<Expression, String> {
        self.next_token();
        let token = self.cur_token.clone();
        let function = Box::from(left_expression);

        let arguments = self.parse_expression_list(TokenType::RParen)?;
        Ok(Expression::Call {
            token,
            function,
            arguments,
        })
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, String> {
        self.next_token();
        let token = self.cur_token.clone();
        self.next_token();

        let index = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RBracket)?;

        Ok(Expression::Index {
            token,
            left: Box::from(left),
            index: Box::from(index),
        })
    }
}

/*
    ==================================================
    LITERAL EXPRESSION PARSING METHODS
    ==================================================
*/
impl Parser {
    fn parse_identifier(&self) -> Expression {
        Expression::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }
    }

    fn parse_literal(&mut self) -> Result<Expression, String> {
        match self.cur_token.tok_type {
            TokenType::True | TokenType::False => Ok(Expression::BoolLiteral {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.parse().unwrap(),
            }),
            TokenType::Int => {
                let int_val = self.cur_token.literal.parse::<isize>();
                if int_val.is_ok() {
                    Ok(Expression::IntLiteral {
                        token: self.cur_token.clone(),
                        value: int_val.unwrap(),
                    })
                } else {
                    Err(format!(
                        "Unable to parse {} as an integer: {}",
                        self.cur_token.literal,
                        int_val.unwrap_err()
                    ))
                }
            }
            TokenType::String => {
                return Ok(Expression::StringLiteral {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.clone(),
                });
            }
            _ => {
                panic!("unreachable")
            }
        }
    }

    fn parse_array_literal(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();

        let elements = self.parse_expression_list(TokenType::RBracket)?;

        Ok(Expression::ArrayLiteral { token, elements })
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();
        let mut entries: HashMap<Expression, Expression> = HashMap::new();

        while !self.peek_token_is(TokenType::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            self.expect_peek(TokenType::Colon)?;
            self.next_token();

            let value = self.parse_expression(Precedence::Lowest)?;

            entries.insert(key, value);

            if !self.peek_token_is(TokenType::RBrace) {
                self.expect_peek(TokenType::Comma)?;
            }
        }

        self.expect_peek(TokenType::RBrace)?;

        Ok(Expression::HashLiteral { token, entries })
    }
}

/*
    ==================================================
    HELPER METHODS
    ==================================================
*/
impl Parser {
    fn parse_infix_inner(&mut self, left_expression: Expression) -> Result<Expression, String> {
        self.next_token();
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();

        let right_expression = self.parse_expression(precedence)?;

        Ok(Expression::Infix {
            token,
            left_expression: Box::new(left_expression),
            operator,
            right_expression: Box::new(right_expression),
        })
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Expression>, String> {
        let mut result = Vec::new();

        if self.peek_token_is(end) {
            self.next_token();
            return Ok(result);
        }

        self.next_token();
        result.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            result.push(self.parse_expression(Precedence::Lowest)?);
        }

        self.expect_peek(end)?;

        Ok(result)
    }
}
