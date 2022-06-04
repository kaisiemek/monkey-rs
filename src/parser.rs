use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
    token::TokenType,
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

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

    fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.cur_token.tok_type {
            TokenType::LET => self.parse_let_statement(),
            _ => Err(()),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ()> {
        let token = self.cur_token.clone();
        if self.expect_peek(TokenType::IDENT).is_err() {
            return Err(());
        }

        let identifier = self.cur_token.literal.clone();
        let value = Expression::IdentifierExpr;

        if self.expect_peek(TokenType::ASSIGN).is_err() {
            return Err(());
        }

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

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}
