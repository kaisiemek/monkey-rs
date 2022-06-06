mod ast;
mod precedence;
pub mod print;
mod test;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};

use self::{
    ast::{Expression, Program, Statement},
    precedence::{get_operator_precedence, Precedence},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
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
            errors: Vec::new(),
        };

        // Initialise parser by setting the cur_ and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, &Vec<String>> {
        let mut program: Program = Vec::new();
        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_ok() {
                program.push(stmt.unwrap());
            }
            self.next_token();
        }

        if !self.errors.is_empty() {
            Err(&self.errors)
        } else {
            Ok(program)
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

    fn expect_peek(&mut self, expected_type: TokenType) -> Result<(), ()> {
        if !self.peek_token_is(expected_type) {
            self.add_error(format!(
                "Expected a token of type {}, but got {} instead",
                expected_type, self.peek_token.tok_type
            ));
            return Err(());
        }

        self.next_token();
        Ok(())
    }

    fn peek_precedence(&self) -> Precedence {
        get_operator_precedence(self.peek_token.tok_type)
    }

    fn current_precedence(&self) -> Precedence {
        get_operator_precedence(self.cur_token.tok_type)
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

/*
    ==================================================
    STATEMENT PARSING METHODS
    ==================================================
*/
impl Parser {
    fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.cur_token.tok_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ()> {
        let token = self.cur_token.clone();
        if self.expect_peek(TokenType::IDENT).is_err() {
            return Err(());
        }

        let identifier = self.cur_token.literal.clone();

        self.expect_peek(TokenType::ASSIGN)?;
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        // TODO: Parse expression
        while !self.cur_token_is(TokenType::SEMICOLON) {
            if self.cur_token_is(TokenType::EOF) {
                return Err(());
            }
            self.next_token();
        }

        Ok(Statement::LetStmt {
            token,
            identifier,
            value,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ()> {
        let token = self.cur_token.clone();

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;
        while !self.cur_token_is(TokenType::SEMICOLON) {
            if self.cur_token_is(TokenType::EOF) {
                return Err(());
            }
            self.next_token();
        }
        Ok(Statement::ReturnStmt { token, value })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::ExpressionStmt { token, expression })
    }
}

/*
    ==================================================
    EXPRESSION PARSING METHODS
    ==================================================
*/
impl Parser {
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ()> {
        let prefix = self.parse_prefix_expression();
        if prefix.is_err() {
            self.add_error(format!(
                "Couldn't parse prefix expression for {}",
                self.cur_token.tok_type
            ));
            return Err(());
        }

        let mut left_expression = prefix.unwrap();
        while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            let infix = self.parse_infix_expression(left_expression);
            if infix.is_err() {
                // Get the left expression back that was moved into the infix parsing function
                // to Box<> it in a InfixExpression object
                left_expression = infix.unwrap_err();
                return Ok(left_expression);
            }

            left_expression = infix.unwrap();
        }

        return Ok(left_expression);
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ()> {
        match self.cur_token.tok_type {
            TokenType::IDENT => Ok(self.parse_identifier()),
            TokenType::INT => Ok(self.parse_literal()?),
            TokenType::TRUE | TokenType::FALSE => Ok(self.parse_literal()?),
            TokenType::BANG | TokenType::MINUS => {
                let token = self.cur_token.clone();
                let operator = self.cur_token.literal.clone();
                self.next_token();
                let right_expression = Box::new(self.parse_expression(Precedence::Prefix)?);
                Ok(Expression::PrefixExpression {
                    token,
                    operator,
                    right_expression,
                })
            }
            TokenType::LPAREN => self.parse_grouped_expression(),
            _ => Err(()),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: Expression,
    ) -> Result<Expression, Expression> {
        match self.peek_token.tok_type {
            TokenType::PLUS
            | TokenType::MINUS
            | TokenType::ASTERISK
            | TokenType::SLASH
            | TokenType::EQ
            | TokenType::NOTEQ
            | TokenType::GT
            | TokenType::LT => {
                self.next_token();
            }
            _ => {
                return Err(left_expression);
            }
        }

        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();

        let expr = self.parse_expression(precedence);
        if expr.is_err() {
            return Err(left_expression);
        }
        let right_expression = Box::new(expr.unwrap());

        Ok(Expression::InfixExpression {
            token,
            left_expression: Box::new(left_expression),
            operator,
            right_expression,
        })
    }

    fn parse_identifier(&self) -> Expression {
        Expression::IdentifierExpr {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }
    }

    fn parse_literal(&mut self) -> Result<Expression, ()> {
        let mut error_msg = String::new();

        match self.cur_token.tok_type {
            TokenType::TRUE | TokenType::FALSE => {
                return Ok(Expression::LiteralBoolExpr {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.parse().unwrap(),
                })
            }
            TokenType::INT => {
                let int_val = self.cur_token.literal.parse::<isize>();
                if int_val.is_ok() {
                    return Ok(Expression::LiteralIntExpr {
                        token: self.cur_token.clone(),
                        value: int_val.unwrap(),
                    });
                } else {
                    error_msg = format!(": {}", int_val.unwrap_err().to_string());
                }
            }
            _ => {}
        }

        self.add_error(format!(
            "Unable to parse {} as an integer or boolean{}",
            self.cur_token.literal, error_msg,
        ));

        Err(())
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ()> {
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RPAREN)?;
        Ok(expression)
    }
}
