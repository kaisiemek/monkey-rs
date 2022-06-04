#[cfg(test)]
mod tests {
    use core::panic;
    use std::collections::VecDeque;

    use crate::{
        ast::{Expression, Program, Statement},
        lexer::Lexer,
        parser::Parser,
        token::Token,
    };

    #[test]
    fn test_let_statements() {
        let input = concat!("let x = 5;\n", "let y = 10;\n", "let foobar = 838383;\n",);
        let mut expected_identifiers: VecDeque<&str> = VecDeque::from_iter(["x", "y", "foobar"]);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parse_program(&mut parser, expected_identifiers.len());
        let mut statements = VecDeque::from_iter(program);

        while !expected_identifiers.is_empty() {
            let expected_identifier = expected_identifiers.pop_front().unwrap();
            let current_stmt = statements.pop_front().unwrap();
            test_let_statement(current_stmt, expected_identifier);
        }
    }

    fn test_let_statement(stmt: Statement, expected_name: &str) {
        let stmt_token: Token;
        let stmt_identifier: String;
        let stmt_value: Expression;

        match stmt {
            Statement::LetStmt {
                token,
                identifier,
                value,
            } => {
                stmt_token = token;
                stmt_identifier = identifier;
                stmt_value = value;
            }
            _ => {
                assert!(false, "Expected a LetStatement, got: {:?}", &stmt);
                panic!();
            }
        }

        assert_eq!(
            stmt_token.literal, "let",
            "The token literal was not 'let', got {} instead",
            stmt_token.literal
        );

        assert_eq!(
            stmt_identifier, expected_name,
            "Expected identifier value {} but got {}",
            expected_name, stmt_identifier
        );
    }

    #[test]
    fn test_return_statements() {
        let input = concat!("return 5;\n", "return 10;\n", "return 838383;\n",);
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program: Program = parse_program(&mut parser, 3);
        let mut statements = VecDeque::from_iter(program);

        while !statements.is_empty() {
            let current_stmt = statements.pop_front().unwrap();
            if let Statement::ReturnStmt { token, .. } = current_stmt {
                assert_eq!(
                    token.literal, "return",
                    "Expected statement literal to be 'return' but got '{}'",
                    token.literal
                );
            } else {
                assert!(false, "Expected ReturnStatement, got {:?}", current_stmt);
            }
        }
    }

    fn parse_program(parser: &mut Parser, expected_statements: usize) -> Program {
        match parser.parse_program() {
            Ok(prog) => {
                assert_eq!(
                    prog.len(),
                    expected_statements,
                    "Expected parser to parse 3 statements, parsed {} instead",
                    prog.len()
                );
                return prog;
            }
            Err(err_vec) => {
                let errs = err_vec.join("\n");
                assert!(
                    false,
                    "The parser encountered {} errors:\n{}",
                    err_vec.len(),
                    errs
                );
                panic!();
            }
        }
    }
}
