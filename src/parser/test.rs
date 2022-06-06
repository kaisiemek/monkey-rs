#[cfg(test)]
mod tests {
    use core::panic;
    use std::collections::VecDeque;

    use crate::{
        lexer::{token::Token, Lexer},
        parser::{
            ast::{Expression, Program, Statement},
            Parser,
        },
    };

    #[test]
    fn test_let_statements() {
        let input = concat!(
            "let x = 5;\n",
            "let y = 10;\n",
            "let foobar = 838383;\n",
            "let lol = kek;\n"
        );
        let mut expected_identifiers: VecDeque<&str> =
            VecDeque::from_iter(["x", "y", "foobar", "lol"]);

        let mut expected_values: VecDeque<&str> = VecDeque::from_iter(["5", "10", "838383", "kek"]);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parse_program(&mut parser, expected_identifiers.len());
        let mut statements = VecDeque::from_iter(program);

        while !expected_identifiers.is_empty() {
            let expected_identifier = expected_identifiers.pop_front().unwrap();
            let expected_value = expected_values.pop_front().unwrap();
            let current_stmt = statements.pop_front().unwrap();
            test_let_statement(current_stmt, expected_identifier, expected_value);
        }
    }

    fn test_let_statement(stmt: Statement, expected_name: &str, expected_value: &str) {
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

        test_literal_expression(stmt_value, expected_value);
    }

    #[test]
    fn test_return_statements() {
        let input = concat!(
            "return 5;\n",
            "return 10;\n",
            "return 838383;\n",
            "return kek;"
        );
        let mut expected_values: VecDeque<&str> = VecDeque::from_iter(["5", "10", "838383", "kek"]);
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program: Program = parse_program(&mut parser, expected_values.len());
        let mut statements = VecDeque::from_iter(program);

        while !statements.is_empty() {
            let current_stmt = statements.pop_front().unwrap();
            let expected_value = expected_values.pop_front().unwrap();

            if let Statement::ReturnStmt { token, value } = current_stmt {
                assert_eq!(
                    token.literal, "return",
                    "Expected statement literal to be 'return' but got '{}'",
                    token.literal
                );
                test_literal_expression(value, expected_value);
            } else {
                assert!(false, "Expected ReturnStatement, got {:?}", current_stmt);
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let stmt = parse_program(&mut parser, 1).pop().unwrap();
        if let Statement::ExpressionStmt { token, expression } = stmt {
            assert_eq!(
                token.literal, "foobar",
                "Expected statement token literal to be 'foobar' but got '{}'",
                token.literal
            );

            if let Expression::IdentifierExpr { token, value } = expression {
                assert_eq!(
                    token.literal, "foobar",
                    "Expected expression token literal to be 'foobar' but got '{}'",
                    token.literal
                );
                assert_eq!(
                    value, "foobar",
                    "Expected expression value to be 'foobar' but got '{}'",
                    token.literal
                );
            } else {
                assert!(false, "Expected IdentifierExpr, got {:?}", expression);
            }
        } else {
            assert!(false, "Expected ExpressionStatement, got {:?}", stmt);
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let stmt = parse_program(&mut parser, 1).pop().unwrap();
        if let Statement::ExpressionStmt { token, expression } = stmt {
            assert_eq!(
                token.literal, "5",
                "Expected statement token literal to be '5' but got '{}'",
                token.literal
            );

            test_integer_literal(expression, 5);
        } else {
            assert!(false, "Expected ExpressionStatement, got {:?}", stmt);
        }
    }

    #[test]
    fn test_boolean_literal_expression() {
        let input = "true;\nfalse;\n";
        let mut expected_values = VecDeque::from_iter(["true", "false"]);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let mut stmts = VecDeque::from(parse_program(&mut parser, expected_values.len()));

        while !stmts.is_empty() {
            let current_stmt = stmts.pop_front().unwrap();
            let expected_value = expected_values.pop_front().unwrap();
            if let Statement::ExpressionStmt {
                token: _,
                expression,
            } = current_stmt
            {
                test_literal_expression(expression, expected_value);
            } else {
                assert!(
                    false,
                    "Expected ExpressionStatement, got {:?}",
                    current_stmt
                );
            }
        }
    }

    #[test]
    fn test_prefix_expressions() {
        let input = concat!("-5;\n", "!5;\n", "-foobar;\n", "!foobar;\n");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let mut expected_operators = VecDeque::from_iter(["-", "!", "-", "!"]);
        let mut expected_values = VecDeque::from_iter(["5", "5", "foobar", "foobar"]);
        let mut statements =
            VecDeque::from_iter(parse_program(&mut parser, expected_operators.len()));

        while !statements.is_empty() {
            let cur_statement = statements.pop_front().unwrap();
            let expected_operator = expected_operators.pop_front().unwrap();
            let expected_value = expected_values.pop_front().unwrap();
            if let Statement::ExpressionStmt {
                token: _,
                expression,
            } = cur_statement
            {
                if let Expression::PrefixExpression {
                    token: _,
                    operator,
                    right_expression,
                } = expression
                {
                    assert_eq!(
                        operator, expected_operator,
                        "Expected operator {} but got {}",
                        expected_operator, operator
                    );
                    test_literal_expression(*right_expression, expected_value);
                } else {
                    assert!(false, "Expected PrefixExpression, got {:?}", expression);
                }
            } else {
                assert!(
                    false,
                    "Expected ExpressionStatement, got {:?}",
                    cur_statement
                );
            }
        }
    }

    #[test]
    fn test_infix_expressions() {
        let input = concat!(
            "5 + 10;\n",
            "6 - 10;\n",
            "7 * 10;\n",
            "8 / 10;\n",
            "9 > 10;\n",
            "10 < 10;\n",
            "11 == 10;\n",
            "12 != 10;\n",
            "foobar != foobar;\n",
            "x > y;\n",
            "x < y;\n",
            "x + y;\n",
        );
        let mut expected_operators = VecDeque::from_iter([
            "+", "-", "*", "/", ">", "<", "==", "!=", "!=", ">", "<", "+",
        ]);
        let mut expected_left_values = VecDeque::from_iter([
            "5", "6", "7", "8", "9", "10", "11", "12", "foobar", "x", "x", "x",
        ]);
        let mut expected_right_values = VecDeque::from_iter([
            "10", "10", "10", "10", "10", "10", "10", "10", "foobar", "y", "y", "y",
        ]);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let mut statements = VecDeque::from(parse_program(&mut parser, expected_operators.len()));

        while !statements.is_empty() {
            let cur_statement = statements.pop_front().unwrap();
            let expected_operator = expected_operators.pop_front().unwrap();
            let expected_left_value = expected_left_values.pop_front().unwrap();
            let expected_right_value = expected_right_values.pop_front().unwrap();
            if let Statement::ExpressionStmt {
                token: _,
                expression,
            } = cur_statement
            {
                test_infix_expression(
                    expression,
                    expected_left_value,
                    expected_operator,
                    expected_right_value,
                );
            } else {
                assert!(
                    false,
                    "Expected ExpressionStatement, got {:?}",
                    cur_statement
                );
            }
        }
    }

    #[test]
    fn test_operator_precedences() {
        struct TestCase<'a> {
            input: &'a str,
            expected: &'a str,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "-a * b",
                expected: "((-a) * b)",
            },
            TestCase {
                input: "!-a",
                expected: "(!(-a))",
            },
            TestCase {
                input: "a + b + c",
                expected: "((a + b) + c)",
            },
            TestCase {
                input: "a + b - c",
                expected: "((a + b) - c)",
            },
            TestCase {
                input: "a * b * c",
                expected: "((a * b) * c)",
            },
            TestCase {
                input: "a * b / c",
                expected: "((a * b) / c)",
            },
            TestCase {
                input: "a + b / c",
                expected: "(a + (b / c))",
            },
            TestCase {
                input: "a + b * c + d / e - f",
                expected: "(((a + (b * c)) + (d / e)) - f)",
            },
            TestCase {
                input: "3 + 4; -5 * 5",
                expected: "(3 + 4)((-5) * 5)",
            },
            TestCase {
                input: "5 > 4 == 3 < 4",
                expected: "((5 > 4) == (3 < 4))",
            },
            TestCase {
                input: "5 < 4 != 3 > 4",
                expected: "((5 < 4) != (3 > 4))",
            },
            TestCase {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            },
        ];

        for test_case in test_cases {
            let lexer = Lexer::new(test_case.input);
            let mut parser = Parser::new(lexer);
            let prog = parser.parse_program();
            match prog {
                Ok(stmts) => {
                    let mut prog_str = String::new();
                    for stmt in stmts {
                        prog_str.push_str(&stmt.to_string());
                    }
                    assert_eq!(test_case.expected, prog_str);
                }
                Err(errs) => {
                    let err_str = errs.join("\n");
                    assert!(
                        false,
                        "The parser encountered {} errors:\n{}",
                        errs.len(),
                        err_str
                    );
                }
            }
        }
    }

    fn test_integer_literal(expression: Expression, expected_value: isize) {
        if let Expression::LiteralIntExpr { token, value } = expression {
            assert_eq!(
                token.literal,
                format!("{}", expected_value),
                "Expected expression token literal to be {} but got '{}'",
                expected_value,
                token.literal
            );
            assert_eq!(
                value, expected_value,
                "Expected expression value to be {} but got {}",
                expected_value, token.literal
            );
        } else {
            assert!(false, "Expected LiteralExpr, got {:?}", expression);
        }
    }

    fn test_identifier(expression: Expression, expected_name: &str) {
        if let Expression::IdentifierExpr { token, value } = expression {
            assert_eq!(
                token.literal,
                format!("{}", expected_name),
                "Expected expression token literal to be {} but got '{}'",
                expected_name,
                token.literal
            );
            assert_eq!(
                value, expected_name,
                "Expected expression value to be {} but got {}",
                expected_name, token.literal
            );
        } else {
            assert!(false, "Expected IdentifierExpr, got {:?}", expression);
        }
    }

    fn test_boolean_literal(expression: Expression, expected_value: bool) {
        if let Expression::LiteralBoolExpr { token, value } = expression {
            assert_eq!(
                token.literal,
                format!("{}", expected_value),
                "Expected expression token literal to be {} but got '{}'",
                expected_value,
                token.literal
            );
            assert_eq!(
                value, expected_value,
                "Expected expression value to be {} but got {}",
                expected_value, token.literal
            );
        } else {
            assert!(false, "Expected LiteralBoolExpr, got {:?}", expression);
        }
    }

    fn test_literal_expression(expression: Expression, expected_value: &str) {
        let value = expected_value.parse::<isize>();
        if value.is_ok() {
            test_integer_literal(expression, value.unwrap());
            return;
        }

        let value = expected_value.parse::<bool>();
        if value.is_ok() {
            test_boolean_literal(expression, value.unwrap());
            return;
        }

        test_identifier(expression, expected_value);
    }

    fn test_infix_expression(
        expression: Expression,
        expected_left: &str,
        expected_op: &str,
        expected_right: &str,
    ) {
        if let Expression::InfixExpression {
            token: _,
            left_expression,
            operator,
            right_expression,
        } = expression
        {
            test_literal_expression(*left_expression, expected_left);
            assert_eq!(
                operator, expected_op,
                "Expected operator {} but got {}",
                expected_op, operator
            );
            test_literal_expression(*right_expression, expected_right);
        } else {
            assert!(false, "Expected IdentifierExpr, got {:?}", expression);
        }
    }

    fn parse_program(parser: &mut Parser, expected_statements: usize) -> Program {
        match parser.parse_program() {
            Ok(prog) => {
                assert_eq!(
                    prog.len(),
                    expected_statements,
                    "Expected parser to parse {} statements, parsed {} instead",
                    expected_statements,
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
