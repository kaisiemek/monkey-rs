#[cfg(test)]
mod test {
    use crate::{
        lexer::{
            token::{Token, TokenType},
            Lexer,
        },
        parser::{
            ast::{Expression, Program, Statement},
            Parser,
        },
    };
    use std::collections::{HashMap, VecDeque};

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

        let program = parse_program(input, expected_identifiers.len());
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
            Statement::Let {
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

        let program: Program = parse_program(input, expected_values.len());
        let mut statements = VecDeque::from_iter(program);

        while !statements.is_empty() {
            let current_stmt = statements.pop_front().unwrap();
            let expected_value = expected_values.pop_front().unwrap();

            if let Statement::Return { token, value } = current_stmt {
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

        let stmt = parse_program(input, 1).pop().unwrap();
        if let Statement::Expression { token, expression } = stmt {
            assert_eq!(
                token.literal, "foobar",
                "Expected statement token literal to be 'foobar' but got '{}'",
                token.literal
            );

            if let Expression::Identifier { token, value } = expression {
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

        let stmt = parse_program(input, 1).pop().unwrap();
        if let Statement::Expression { token, expression } = stmt {
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

        let mut stmts = VecDeque::from(parse_program(input, expected_values.len()));

        while !stmts.is_empty() {
            let current_stmt = stmts.pop_front().unwrap();
            let expected_value = expected_values.pop_front().unwrap();
            if let Statement::Expression {
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
        struct TestCase<'a> {
            input: &'a str,
            expected_operator: &'a str,
            expected_right_value: &'a str,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "-5;",
                expected_operator: "-",
                expected_right_value: "5",
            },
            TestCase {
                input: "!5;",
                expected_operator: "!",
                expected_right_value: "5",
            },
            TestCase {
                input: "-foobar;",
                expected_operator: "-",
                expected_right_value: "foobar",
            },
            TestCase {
                input: "!foobar;",
                expected_operator: "!",
                expected_right_value: "foobar",
            },
            TestCase {
                input: "!true;",
                expected_operator: "!",
                expected_right_value: "true",
            },
            TestCase {
                input: "!false;",
                expected_operator: "!",
                expected_right_value: "false",
            },
        ];

        for test_case in test_cases {
            let expression = parse_expression_statement(test_case.input);
            if let Expression::Prefix {
                token: _,
                operator,
                right_expression,
            } = expression
            {
                assert_eq!(
                    operator, test_case.expected_operator,
                    "Expected operator {} but got {}",
                    test_case.expected_operator, operator
                );
                test_literal_expression(*right_expression, test_case.expected_right_value);
            } else {
                assert!(false, "Expected PrefixExpression, got {:?}", expression);
            }
        }
    }

    #[test]
    fn test_infix_expressions() {
        struct TestCase<'a> {
            input: &'a str,
            expected_left_value: &'a str,
            expected_operator: &'a str,
            expected_right_value: &'a str,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "5 + 10;\n",
                expected_operator: "+",
                expected_left_value: "5",
                expected_right_value: "10",
            },
            TestCase {
                input: "6 - 10;\n",
                expected_operator: "-",
                expected_left_value: "6",
                expected_right_value: "10",
            },
            TestCase {
                input: "7 * 10;\n",
                expected_operator: "*",
                expected_left_value: "7",
                expected_right_value: "10",
            },
            TestCase {
                input: "8 / 10;\n",
                expected_operator: "/",
                expected_left_value: "8",
                expected_right_value: "10",
            },
            TestCase {
                input: "9 > 10;\n",
                expected_operator: ">",
                expected_left_value: "9",
                expected_right_value: "10",
            },
            TestCase {
                input: "10 < 10;\n",
                expected_operator: "<",
                expected_left_value: "10",
                expected_right_value: "10",
            },
            TestCase {
                input: "11 == 10;\n",
                expected_operator: "==",
                expected_left_value: "11",
                expected_right_value: "10",
            },
            TestCase {
                input: "12 != 10;\n",
                expected_operator: "!=",
                expected_left_value: "12",
                expected_right_value: "10",
            },
            TestCase {
                input: "foobar != foobar;\n",
                expected_operator: "!=",
                expected_left_value: "foobar",
                expected_right_value: "foobar",
            },
            TestCase {
                input: "x > y;\n",
                expected_operator: ">",
                expected_left_value: "x",
                expected_right_value: "y",
            },
            TestCase {
                input: "x < y;\n",
                expected_operator: "<",
                expected_left_value: "x",
                expected_right_value: "y",
            },
            TestCase {
                input: "x + y;\n",
                expected_operator: "+",
                expected_left_value: "x",
                expected_right_value: "y",
            },
            TestCase {
                input: "true == true;\n",
                expected_operator: "==",
                expected_left_value: "true",
                expected_right_value: "true",
            },
            TestCase {
                input: "true != false;\n",
                expected_operator: "!=",
                expected_left_value: "true",
                expected_right_value: "false",
            },
            TestCase {
                input: "false == false;\n",
                expected_operator: "==",
                expected_left_value: "false",
                expected_right_value: "false",
            },
        ];

        for test_case in test_cases {
            let expression = parse_expression_statement(test_case.input);
            test_infix_expression(
                expression,
                test_case.expected_left_value,
                test_case.expected_operator,
                test_case.expected_right_value,
            );
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
                input: "-a * b;",
                expected: "((-a) * b)",
            },
            TestCase {
                input: "!-a;",
                expected: "(!(-a))",
            },
            TestCase {
                input: "a + b + c;",
                expected: "((a + b) + c)",
            },
            TestCase {
                input: "a + b - c;",
                expected: "((a + b) - c)",
            },
            TestCase {
                input: "a * b * c;",
                expected: "((a * b) * c)",
            },
            TestCase {
                input: "a * b / c;",
                expected: "((a * b) / c)",
            },
            TestCase {
                input: "a + b / c;",
                expected: "(a + (b / c))",
            },
            TestCase {
                input: "a + b * c + d / e - f;",
                expected: "(((a + (b * c)) + (d / e)) - f)",
            },
            TestCase {
                input: "3 + 4 - -5 * 5;",
                expected: "((3 + 4) - ((-5) * 5))",
            },
            TestCase {
                input: "5 > 4 == 3 < 4;",
                expected: "((5 > 4) == (3 < 4))",
            },
            TestCase {
                input: "5 < 4 != 3 > 4;",
                expected: "((5 < 4) != (3 > 4))",
            },
            TestCase {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            },
            TestCase {
                input: "true;",
                expected: "true",
            },
            TestCase {
                input: "false;",
                expected: "false",
            },
            TestCase {
                input: "3 > 5 == false;",
                expected: "((3 > 5) == false)",
            },
            TestCase {
                input: "3 < 5 == true;",
                expected: "((3 < 5) == true)",
            },
            TestCase {
                input: "1 + (2 + 3) + 4;",
                expected: "((1 + (2 + 3)) + 4)",
            },
            TestCase {
                input: "1 + (2 + 3) * 4;",
                expected: "(1 + ((2 + 3) * 4))",
            },
            TestCase {
                input: "(5 + 5) * 2;",
                expected: "((5 + 5) * 2)",
            },
            TestCase {
                input: "2 / (5 + 5);",
                expected: "(2 / (5 + 5))",
            },
            TestCase {
                input: "-(5 + 5);",
                expected: "(-(5 + 5))",
            },
            TestCase {
                input: "!(true == true);",
                expected: "(!(true == true))",
            },
            TestCase {
                input: "a + add(b * c) + d",
                expected: "((a + add((b * c))) + d)",
            },
            TestCase {
                input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            },
            TestCase {
                input: "add(a + b + c * d / f + g)",
                expected: "add((((a + b) + ((c * d) / f)) + g))",
            },
            TestCase {
                input: "a * [1, 2, 3, 4][b * c] * d",
                expected: "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            },
            TestCase {
                input: "add(a * b[2], b[1], 2 * [1, 2][1])",
                expected: "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            },
        ];

        for test_case in test_cases {
            let stmt = parse_program(test_case.input, 1).pop().unwrap();
            assert_eq!(test_case.expected, stmt.to_string());
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";

        let if_expression = parse_expression_statement(input);

        let if_condition;
        let if_consequence;
        let if_alternative;
        if let Expression::If {
            token: _,
            condition,
            consequence,
            alternative,
        } = if_expression
        {
            if_condition = condition;
            if_consequence = consequence;
            if_alternative = alternative;
        } else {
            assert!(false, "Expected IfExpression, got {:?}", if_expression);
            panic!();
        }

        test_infix_expression(*if_condition, "x", "<", "y");
        assert_eq!(
            if_consequence.statements.len(),
            1,
            "Expected only one statement in the consequence block"
        );

        let consequence_expression;
        if let Statement::Expression {
            token: _,
            expression,
        } = &if_consequence.statements[0]
        {
            consequence_expression = expression.clone();
        } else {
            assert!(
                false,
                "Expected ExpressionStatement in consequence block, got {:?}",
                &if_consequence.statements[0]
            );
            panic!();
        }

        test_identifier(consequence_expression, "x");
        assert!(
            if_alternative.is_none(),
            "Did expect else block to be empty"
        );
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";

        let if_expression = parse_expression_statement(input);

        let if_condition;
        let if_consequence;
        let if_alternative;
        if let Expression::If {
            token: _,
            condition,
            consequence,
            alternative,
        } = if_expression
        {
            if_condition = condition;
            if_consequence = consequence;
            if_alternative = alternative;
        } else {
            assert!(false, "Expected IfExpression, got {:?}", if_expression);
            panic!();
        }

        test_infix_expression(*if_condition, "x", "<", "y");

        assert_eq!(
            if_consequence.statements.len(),
            1,
            "Expected only one statement in the consequence block"
        );
        let consequence_expression;
        if let Statement::Expression {
            token: _,
            expression,
        } = &if_consequence.statements[0]
        {
            consequence_expression = expression.clone();
        } else {
            assert!(
                false,
                "Expected ExpressionStatement in consequence block, got {:?}",
                &if_consequence.statements[0]
            );
            panic!();
        }
        test_identifier(consequence_expression, "x");

        assert!(if_alternative.is_some());
        let alternative = if_alternative.unwrap();
        assert_eq!(
            alternative.statements.len(),
            1,
            "Expected only one statement in the consequence block"
        );

        let alternative_expression;
        if let Statement::Expression {
            token: _,
            expression,
        } = &alternative.statements[0]
        {
            alternative_expression = expression.clone();
        } else {
            assert!(
                false,
                "Expected ExpressionStatement in consequence block, got {:?}",
                &if_consequence.statements[0]
            );
            panic!();
        }

        test_identifier(alternative_expression, "y");
    }

    #[test]
    fn test_function_literal_expression() {
        let input = "fn(x, y) { x + y; }";

        let expression = parse_expression_statement(input);

        let Expression::FnLiteral {
            token: _,
            parameters,
            body,
        } = expression else {
            assert!(false, "Expected LiteralFnExpr, got {:?}", expression);
            panic!();
        };

        assert_eq!(parameters.len(), 2, "Expected 2 parameters");
        assert_eq!(parameters[0], "x");
        assert_eq!(parameters[1], "y");

        let body_expression;
        if let Statement::Expression {
            token: _,
            expression,
        } = &body.statements[0]
        {
            body_expression = expression.clone();
        } else {
            panic!(
                "Expected ExpressioStmt in body, got {:?}",
                body.statements[0]
            );
        }

        test_infix_expression(body_expression, "x", "+", "y");
    }

    #[test]
    fn test_function_parameters() {
        struct TestCase<'a> {
            input: &'a str,
            expected_parameters: Vec<&'a str>,
        }

        let test_cases = vec![
            TestCase {
                input: "fn() {};",
                expected_parameters: vec![],
            },
            TestCase {
                input: "fn(x) {};",
                expected_parameters: vec!["x"],
            },
            TestCase {
                input: "fn(x, y, z) {};",
                expected_parameters: vec!["x", "y", "z"],
            },
            TestCase {
                input: "fn(a, b, c, d, e, f, g, h) {};",
                expected_parameters: vec!["a", "b", "c", "d", "e", "f", "g", "h"],
            },
        ];

        for test_case in test_cases {
            let expression = parse_expression_statement(test_case.input);

            if let Expression::FnLiteral {
                token: _,
                parameters,
                ..
            } = expression
            {
                assert_eq!(
                    parameters.len(),
                    test_case.expected_parameters.len(),
                    "Expected {} parameters but got {}",
                    test_case.expected_parameters.len(),
                    parameters.len()
                );

                for (i, param) in parameters.iter().enumerate() {
                    assert_eq!(param, test_case.expected_parameters[i]);
                }
            } else {
                assert!(false, "Expected LiteralFnExpr, got {:?}", expression);
            }
        }
    }

    #[test]
    fn test_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5, true == false)";
        let expression = parse_expression_statement(input);

        if let Expression::Call {
            token: _,
            function,
            arguments,
        } = expression
        {
            test_identifier(*function, "add");
            assert_eq!(
                arguments.len(),
                4,
                "Expected 4 arguments, got {}",
                arguments.len()
            );
            test_literal_expression(arguments[0].clone(), "1");
            test_infix_expression(arguments[1].clone(), "2", "*", "3");
            test_infix_expression(arguments[2].clone(), "4", "+", "5");
            test_infix_expression(arguments[3].clone(), "true", "==", "false");
        } else {
            assert!(false, "Expected CallExpr, got {:?}", expression);
            panic!();
        }
        // let statement = parse_program(&mut parser, expected_statements)
    }

    #[test]
    fn test_call_arguments() {
        struct TestCase<'a> {
            input: &'a str,
            expected_arguments: Vec<&'a str>,
        }

        let test_cases = vec![
            TestCase {
                input: "add();",
                expected_arguments: vec![],
            },
            TestCase {
                input: "add(x);",
                expected_arguments: vec!["x"],
            },
            TestCase {
                input: "add(x, y, z);",
                expected_arguments: vec!["x", "y", "z"],
            },
            TestCase {
                input: "add(a, b, c, d, e, f, g, h);",
                expected_arguments: vec!["a", "b", "c", "d", "e", "f", "g", "h"],
            },
        ];

        for test_case in test_cases {
            let expression = parse_expression_statement(test_case.input);
            if let Expression::Call {
                token: _,
                arguments,
                function,
            } = expression
            {
                test_identifier(*function, "add");
                assert_eq!(
                    arguments.len(),
                    test_case.expected_arguments.len(),
                    "Expected {} parameters but got {}",
                    test_case.expected_arguments.len(),
                    arguments.len()
                );

                for (i, param) in arguments.iter().enumerate() {
                    test_literal_expression(param.clone(), test_case.expected_arguments[i]);
                }
            } else {
                assert!(false, "Expected CallExpr, got {:?}", expression);
            }
        }
    }

    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello world\"";
        let expr = parse_expression_statement(input);
        if let Expression::StringLiteral { token: _, value } = expr {
            assert_eq!(value, "hello world");
        } else {
            assert!(false, "Expected LiteralStringExpr, got {:?}", expr);
        }
    }

    #[test]
    fn test_array_literal() {
        let input = "[1, 2 * 2, 3 + 3]";

        let expression = parse_expression_statement(input);
        if let Expression::ArrayLiteral {
            token: _,
            elements: value,
        } = expression
        {
            assert_eq!(value.len(), 3);
            test_integer_literal(value[0].clone(), 1);
            test_infix_expression(value[1].clone(), "2", "*", "2");
            test_infix_expression(value[2].clone(), "3", "+", "3");
        } else {
            panic!("Expected LiteralArrayExpr, got {:?}", expression);
        }
    }

    #[test]
    fn test_index_expression() {
        let input = "arr[1 + 1]";
        let expression = parse_expression_statement(input);

        if let Expression::Index {
            token: _,
            left,
            index,
        } = expression
        {
            test_identifier(*left, "arr");
            test_infix_expression(*index, "1", "+", "1");
        } else {
            panic!("Expected IndexExpr, got {:?}", expression);
        }
    }

    #[test]
    fn test_hash_literal_string_key() {
        let input = "{\"one\": 1, \"two\": 2, \"three\": 3}";
        let expected: HashMap<String, isize> = HashMap::from([
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ("three".to_string(), 3),
        ]);

        let expression = parse_expression_statement(input);

        if let Expression::HashLiteral { token: _, entries } = expression {
            assert_eq!(entries.len(), 3);
            for (key, val) in entries {
                {
                    if let Expression::IntLiteral { token: _, value } = val {
                        let expected_val = expected.get(&key.to_string()).unwrap().to_owned();
                        assert_eq!(value, expected_val);
                    } else {
                        panic!("Expected LiteralIntExpr, got {:?}", val)
                    }
                }
            }
        } else {
            panic!("Expected LiteralHashExpr, got {:?}", expression);
        }
    }

    #[test]
    fn test_empty_hash_literal() {
        let input = "{}";
        let expression = parse_expression_statement(input);

        if let Expression::HashLiteral { token: _, entries } = expression {
            assert_eq!(entries.len(), 0);
        } else {
            panic!("Expected LiteralHashExpr, got {:?}", expression);
        }
    }

    #[test]
    fn test_hash_literal_expressions() {
        let input = "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}";
        let expression = parse_expression_statement(input);

        let one = Expression::StringLiteral {
            token: Token::new(TokenType::String, "one"),
            value: "one".to_string(),
        };
        let two = Expression::StringLiteral {
            token: Token::new(TokenType::String, "two"),
            value: "two".to_string(),
        };
        let three = Expression::StringLiteral {
            token: Token::new(TokenType::String, "three"),
            value: "three".to_string(),
        };

        if let Expression::HashLiteral { token: _, entries } = expression {
            assert_eq!(entries.len(), 3);
            let val1 = entries.get(&one).unwrap().clone();
            let val2 = entries.get(&two).unwrap().clone();
            let val3 = entries.get(&three).unwrap().clone();
            test_infix_expression(val1, "0", "+", "1");
            test_infix_expression(val2, "10", "-", "8");
            test_infix_expression(val3, "15", "/", "5");
        } else {
            panic!("Epected LiteralHashExpr, got {:?}", expression);
        }
    }

    fn test_integer_literal(expression: Expression, expected_value: isize) {
        if let Expression::IntLiteral { token, value } = expression {
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
        if let Expression::Identifier { token, value } = expression {
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
        if let Expression::BoolLiteral { token, value } = expression {
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
        expected_operator: &str,
        expected_right: &str,
    ) {
        if let Expression::Infix {
            token: _,
            left_expression,
            operator,
            right_expression,
        } = expression
        {
            test_literal_expression(*left_expression, expected_left);
            assert_eq!(
                operator, expected_operator,
                "Expected operator {} but got {}",
                expected_operator, operator
            );
            test_literal_expression(*right_expression, expected_right);
        } else {
            assert!(false, "Expected IdentifierExpr, got {:?}", expression);
        }
    }

    fn parse_expression_statement(input: &str) -> Expression {
        let statement = parse_program(input, 1).pop().unwrap();
        if let Statement::Expression {
            token: _,
            expression,
        } = statement
        {
            return expression;
        } else {
            assert!(false, "Expected ExpressionStmt, got {:?}", statement);
            panic!();
        }
    }

    fn parse_program(input: &str, expected_statements: usize) -> Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

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
