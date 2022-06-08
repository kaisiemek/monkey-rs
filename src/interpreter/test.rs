#[cfg(test)]
mod test {
    use crate::{
        interpreter::{eval, object::Object},
        lexer::Lexer,
        parser::{ast::Node, Parser},
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
        ];

        for test_case in test_cases {
            let obj = test_eval(test_case.input);
            test_bool_object(obj, test_case.expected);
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        match parser.parse_program() {
            Ok(program) => {
                return eval(Node::Program(program));
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
}
