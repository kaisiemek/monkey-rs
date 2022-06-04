#[cfg(test)]
mod tests {
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

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program: Program;
        match parser.parse_program() {
            Ok(prog) => program = prog,
            Err(err_vec) => {
                let errs = err_vec.join("\n");
                assert!(
                    false,
                    "The parser encountered {} errors:\n{}",
                    err_vec.len(),
                    errs
                );
                return;
            }
        }

        assert_eq!(
            program.len(),
            3,
            "Expected parser to parse 3 statements, parsed {} instead",
            program.len()
        );

        let mut expected_identifiers: VecDeque<&str> = VecDeque::from_iter(["x", "y", "foobar"]);
        let mut statements = VecDeque::from_iter(program);

        while !expected_identifiers.is_empty() {
            let expected_identifier = expected_identifiers.pop_front().unwrap();
            let current_stmt = statements.pop_front().unwrap();

            let result = test_let_statement(current_stmt, expected_identifier);
            assert!(matches!(result, Ok(())));
        }
    }

    fn test_let_statement(stmt: Statement, expected_name: &str) -> Result<(), String> {
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
                return Err(String::from(
                    "Statement was not an instance of a let statement",
                ));
            }
        };

        if stmt_token.literal != "let" {
            return Err(format!(
                "The token literal was not 'let', got {} instead",
                stmt_token.literal
            ));
        }

        if stmt_identifier != expected_name {
            return Err(format!(
                "Expected identifier value {} but got {}",
                expected_name, stmt_identifier
            ));
        }

        Ok(())
    }
}
