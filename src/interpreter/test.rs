#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        interpreter::{
            environment::Environment,
            eval_program,
            object::{Inspectable, Object},
        },
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_integer_expression() {
        struct TestCase<'a> {
            input: &'a str,
            expected: isize,
        }

        let test_cases = vec![
            TestCase {
                input: "5",
                expected: 5,
            },
            TestCase {
                input: "10",
                expected: 10,
            },
            TestCase {
                input: "-5",
                expected: -5,
            },
            TestCase {
                input: "-10",
                expected: -10,
            },
            TestCase {
                input: "5 + 5 + 5 + 5 - 10",
                expected: 10,
            },
            TestCase {
                input: "2 * 2 * 2 * 2 * 2",
                expected: 32,
            },
            TestCase {
                input: "-50 + 100 + -50",
                expected: 0,
            },
            TestCase {
                input: "5 * 2 + 10",
                expected: 20,
            },
            TestCase {
                input: "5 + 2 * 10",
                expected: 25,
            },
            TestCase {
                input: "20 + 2 * -10",
                expected: 0,
            },
            TestCase {
                input: "50 / 2 * 2 + 10",
                expected: 60,
            },
            TestCase {
                input: "2 * (5 + 10)",
                expected: 30,
            },
            TestCase {
                input: "3 * 3 * 3 + 10",
                expected: 37,
            },
            TestCase {
                input: "3 * (3 * 3) + 10",
                expected: 37,
            },
            TestCase {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                expected: 50,
            },
        ];

        for test_case in test_cases {
            let obj = test_eval(test_case.input);
            test_integer_object(obj, test_case.expected);
        }
    }

    #[test]
    fn test_boolean_expression() {
        struct TestCase<'a> {
            input: &'a str,
            expected: bool,
        }

        let test_cases = vec![
            TestCase {
                input: "true",
                expected: true,
            },
            TestCase {
                input: "false",
                expected: false,
            },
            TestCase {
                input: "1 < 2",
                expected: true,
            },
            TestCase {
                input: "1 > 2",
                expected: false,
            },
            TestCase {
                input: "1 < 1",
                expected: false,
            },
            TestCase {
                input: "1 > 1",
                expected: false,
            },
            TestCase {
                input: "1 == 1",
                expected: true,
            },
            TestCase {
                input: "1 != 1",
                expected: false,
            },
            TestCase {
                input: "1 == 2",
                expected: false,
            },
            TestCase {
                input: "1 != 2",
                expected: true,
            },
            TestCase {
                input: "true == true",
                expected: true,
            },
            TestCase {
                input: "false == false",
                expected: true,
            },
            TestCase {
                input: "true == false",
                expected: false,
            },
            TestCase {
                input: "true != false",
                expected: true,
            },
            TestCase {
                input: "false != true",
                expected: true,
            },
            TestCase {
                input: "(1 < 2) == true",
                expected: true,
            },
            TestCase {
                input: "(1 < 2) == false",
                expected: false,
            },
            TestCase {
                input: "(1 > 2) == true",
                expected: false,
            },
            TestCase {
                input: "(1 > 2) == false",
                expected: true,
            },
        ];

        for test_case in test_cases {
            let obj = test_eval(test_case.input);
            test_bool_object(obj, test_case.expected);
        }
    }

    #[test]
    fn test_if_else_expression() {
        struct TestCase<'a> {
            input: &'a str,
            expected: Object,
        }

        let test_cases = vec![
            TestCase {
                input: "if (true) { 10 }",
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (false) { 10 }",
                expected: Object::Null,
            },
            TestCase {
                input: "if (1) { 10 }",
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (1 < 2) { 10 }",
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (1 > 2) { 10 }",
                expected: Object::Null,
            },
            TestCase {
                input: "if (1 > 2) { 10 } else { 20 }",
                expected: Object::Integer(20),
            },
            TestCase {
                input: "if (1 < 2) { 10 } else { 20 }",
                expected: Object::Integer(10),
            },
        ];

        for test_case in test_cases {
            let obj = test_eval(test_case.input);
            assert_eq!(obj.inspect(), test_case.expected.inspect());
        }
    }

    #[test]
    fn test_bang_operator() {
        struct TestCase<'a> {
            input: &'a str,
            expected: bool,
        }

        let test_cases = vec![
            TestCase {
                input: "!true",
                expected: false,
            },
            TestCase {
                input: "!false",
                expected: true,
            },
            TestCase {
                input: "!5",
                expected: false,
            },
            TestCase {
                input: "!!true",
                expected: true,
            },
            TestCase {
                input: "!!false",
                expected: false,
            },
            TestCase {
                input: "!!5",
                expected: true,
            },
        ];

        for test_case in test_cases {
            let obj = test_eval(test_case.input);
            test_bool_object(obj, test_case.expected);
        }
    }

    #[test]
    fn test_return_statement() {
        struct TestCase<'a> {
            input: &'a str,
            expected: isize,
        }

        let test_cases = vec![
            TestCase {
                input: "return 10;",
                expected: 10,
            },
            TestCase {
                input: "return 10; 9;",
                expected: 10,
            },
            TestCase {
                input: "return 2 * 5; 9;",
                expected: 10,
            },
            TestCase {
                input: "9; return 2 * 5; 9;",
                expected: 10,
            },
            TestCase {
                input: "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
                expected: 10,
            },
        ];

        for test_case in test_cases {
            let obj = test_eval(test_case.input);
            test_integer_object(obj, test_case.expected);
        }
    }

    #[test]
    fn test_error_handling() {
        struct TestCase<'a> {
            input: &'a str,
            expected: &'a str,
        }

        let test_cases = vec![
            TestCase {
                input: "5 + true;",
                expected: "type mismatch: INTEGER + BOOLEAN",
            },
            TestCase {
                input: "5 + true; 5;",
                expected: "type mismatch: INTEGER + BOOLEAN",
            },
            TestCase {
                input: "-true",
                expected: "unknown operator: -BOOLEAN",
            },
            TestCase {
                input: "true + false;",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            TestCase {
                input: "5; true + false; 5",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            TestCase {
                input: "if (10 > 1) { true + false }",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            TestCase {
                input: concat!(
                    "if (10 > 1) {",
                    "  if (10 > 1) {",
                    "    return true + false;",
                    "  }",
                    "  return 1;",
                    "}",
                ),
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            TestCase {
                input: "foobar",
                expected: "unknown identifier: foobar",
            },
            TestCase {
                input: "\"Hello\" - \"World\"",
                expected: "unknown operator: STRING - STRING",
            },
            TestCase {
                input: "\"Hello\" * \"World\"",
                expected: "unknown operator: STRING * STRING",
            },
            TestCase {
                input: "-\"Hello\"",
                expected: "unknown operator: -STRING",
            },
        ];

        for test_case in test_cases {
            let msg = test_eval_error(test_case.input);
            assert_eq!(msg, test_case.expected);
        }
    }

    #[test]
    fn test_let_statements() {
        struct TestCase<'a> {
            input: &'a str,
            expected: isize,
        }

        let test_cases = vec![
            TestCase {
                input: "let a = 5; a;",
                expected: 5,
            },
            TestCase {
                input: "let a = 5 * 5; a;",
                expected: 25,
            },
            TestCase {
                input: "let a = 5; let b = a; b;",
                expected: 5,
            },
            TestCase {
                input: "let a = 5; let b = a; let c = a + b + 5; c;",
                expected: 15,
            },
        ];

        for test_case in test_cases {
            let object = test_eval(test_case.input);
            test_integer_object(object, test_case.expected);
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let environment = Rc::new(RefCell::new(Environment::new()));

        match parser.parse_program() {
            Ok(program) => {
                let object = eval_program(program, environment);
                match object {
                    Ok(obj) => obj,
                    Err(msg) => {
                        assert!(
                            false,
                            "An error occured while evaluating the program: {}",
                            msg
                        );
                        panic!();
                    }
                }
            }
            Err(errors) => {
                let error_str = errors.join("\n");
                assert!(
                    false,
                    "The parser encountered {} errors:\n{}",
                    errors.len(),
                    error_str
                );
                panic!();
            }
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 1; };";
        let object = test_eval(input);

        assert_eq!(object.type_str(), "FUNCTION");
        println!("{}", object.inspect());
        if let Object::Function {
            parameters,
            body,
            environment: _,
        } = object
        {
            assert_eq!(parameters.len(), 1);
            assert_eq!(parameters[0].to_string(), "x");
            assert_eq!(body.to_string(), "{\n\t(x + 1)\n}");
        } else {
            assert_eq!(object.type_str(), "FUNCTION");
            panic!("how?");
        }
    }

    #[test]
    fn test_function_application() {
        struct TestCase<'a> {
            input: &'a str,
            expected: isize,
        }

        let test_cases = vec![
            TestCase {
                input: "let identity = fn(x) { x; }; identity(5);",
                expected: 5,
            },
            TestCase {
                input: "let identity = fn(x) { return x; }; identity(5);",
                expected: 5,
            },
            TestCase {
                input: "let double = fn(x) { x * 2; }; double(5);",
                expected: 10,
            },
            TestCase {
                input: "let add = fn(x, y) { x + y; }; add(5, 5);",
                expected: 10,
            },
            TestCase {
                input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                expected: 20,
            },
            TestCase {
                input: "fn(x) { x; }(5)",
                expected: 5,
            },
        ];

        for test_case in test_cases {
            let object = test_eval(test_case.input);
            test_integer_object(object, test_case.expected);
        }
    }

    #[test]
    fn test_closures() {
        let input = "
        let newAdder = fn(x) {
            fn(y) { x + y } 
        };
        let addTwo = newAdder(2);
        addTwo(2);";

        let object = test_eval(input);
        test_integer_object(object, 4);
    }

    #[test]
    fn test_strings() {
        struct TestCase<'a> {
            input: &'a str,
            expected: &'a str,
        }
        let test_cases = vec![
            TestCase {
                input: "\"foobar\"",
                expected: "foobar",
            },
            TestCase {
                input: "let x = \"foobar\"; x",
                expected: "foobar",
            },
            TestCase {
                input: "\"foobar\" + \"_test\"",
                expected: "foobar_test",
            },
            TestCase {
                input: "let x = \"foo\"; let y = \"bar\"; x + y;",
                expected: "foobar",
            },
        ];

        for test_case in test_cases {
            let object = test_eval(test_case.input);
            test_string_object(object, test_case.expected);
        }
    }

    #[test]
    fn test_builtins() {
        enum Expected<'a> {
            Error(&'a str),
            Value(isize),
        }
        struct TestCase<'a> {
            input: &'a str,
            expected: Expected<'a>,
        }

        let test_cases = vec![
            TestCase {
                input: "len(\"\")",
                expected: Expected::Value(0),
            },
            TestCase {
                input: "len(\"four\")",
                expected: Expected::Value(4),
            },
            TestCase {
                input: "len(\"hello world\")",
                expected: Expected::Value(11),
            },
            TestCase {
                input: "len(1)",
                expected: Expected::Error("argument to len not supported, got=INTEGER"),
            },
            TestCase {
                input: "len(\"one\", \"two\")",
                expected: Expected::Error("wrong number of arguments. got=2, want=1"),
            },
        ];

        for test_case in test_cases {
            match test_case.expected {
                Expected::Error(msg) => {
                    let err = test_eval_error(test_case.input);
                    assert_eq!(err, msg);
                }
                Expected::Value(val) => {
                    let obj = test_eval(test_case.input);
                    test_integer_object(obj, val);
                }
            }
        }
    }

    #[test]
    fn test_array() {
        let input = "[1, 3 * 4, 1 + 1 + 1 + 1 + 1, \"foo\" + \"bar\", true == false]";

        let obj = test_eval(input);
        if let Object::Array(elements) = obj {
            assert_eq!(elements.len(), 5);
            test_integer_object(elements[0].clone(), 1);
            test_integer_object(elements[1].clone(), 12);
            test_integer_object(elements[2].clone(), 5);
            test_string_object(elements[3].clone(), "foobar");
            test_bool_object(elements[4].clone(), false);
        } else {
            panic!("Expected array object, got {}", obj.inspect());
        }
    }

    #[test]
    fn test_array_index_expressions() {
        struct TestCase<'a> {
            input: &'a str,
            expected: isize,
        }

        let test_cases = vec![
            TestCase {
                input: "[1, 2, 3][0]",
                expected: 1,
            },
            TestCase {
                input: "[1, 2, 3][1]",
                expected: 2,
            },
            TestCase {
                input: "[1, 2, 3][2]",
                expected: 3,
            },
            TestCase {
                input: "let i = 0; [1][i];",
                expected: 1,
            },
            TestCase {
                input: "[1, 2, 3][1 + 1];",
                expected: 3,
            },
            TestCase {
                input: "let myArray = [1, 2, 3]; myArray[2];",
                expected: 3,
            },
            TestCase {
                input: "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                expected: 6,
            },
            TestCase {
                input: "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                expected: 2,
            },
        ];

        for test_case in test_cases {
            let object = test_eval(test_case.input);
            test_integer_object(object, test_case.expected);
        }

        let test_cases2 = vec!["[1, 2, 3][3]", "[1, 2, 3][-1]"];

        for test_case in test_cases2 {
            let object = test_eval(test_case);
            assert_eq!(object.type_str(), "NULL");
        }
    }

    fn test_eval_error(input: &str) -> String {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let environment = Rc::new(RefCell::new(Environment::new()));

        match parser.parse_program() {
            Ok(program) => {
                let object = eval_program(program, environment);
                match object {
                    Ok(obj) => {
                        assert!(
                            false,
                            "Expected an error to occur, but it didn't. Eval returned: {}",
                            obj.inspect()
                        );
                        panic!();
                    }
                    Err(msg) => msg,
                }
            }
            Err(errors) => {
                let error_str = errors.join("\n");
                assert!(
                    false,
                    "The parser encountered {} errors:\n{}",
                    errors.len(),
                    error_str
                );
                panic!();
            }
        }
    }

    fn test_integer_object(object: Object, expected: isize) {
        if let Object::Integer(value) = object {
            assert_eq!(
                value, expected,
                "Expected integer object to contain value {} but got {}",
                expected, value
            )
        } else {
            assert!(false, "Expected Integer object, got {:?}", object);
        }
    }

    fn test_bool_object(object: Object, expected: bool) {
        if let Object::Boolean(value) = object {
            assert_eq!(
                value, expected,
                "Expected boolean object to contain value {} but got {}",
                expected, value
            )
        } else {
            assert!(false, "Expected Boolean object, got {:?}", object);
        }
    }

    fn test_string_object(object: Object, expected: &str) {
        if let Object::String(value) = object {
            assert_eq!(
                value, expected,
                "Expected string object to contain value {} but got {}",
                expected, value
            )
        } else {
            assert!(false, "Expected String object, got {:?}", object);
        }
    }
}
