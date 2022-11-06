#[cfg(test)]
mod test {
    use crate::{
        code::{make, stringify, Instructions, Opcode},
        compiler::Compiler,
        interpreter::object::{Inspectable, Object},
        lexer::Lexer,
        parser::{ast::Program, Parser},
    };

    struct TestCase {
        input: String,
        expected_constants: Vec<Object>,
        expected_instructions: Vec<Instructions>,
    }

    #[test]
    fn test_integer_arithmetic() {
        let test_cases = vec![
            TestCase {
                input: "1 + 2".to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]), // index of constant 1
                    make(Opcode::Constant, vec![1]), // index of constant 2
                    make(Opcode::Add, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "1 - 2".to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]), // index of constant 1
                    make(Opcode::Constant, vec![1]), // index of constant 2
                    make(Opcode::Sub, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "3 * 2".to_string(),
                expected_constants: vec![Object::Integer(3), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]), // index of constant 3
                    make(Opcode::Constant, vec![1]), // index of constant 2
                    make(Opcode::Mult, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "6 / 2".to_string(),
                expected_constants: vec![Object::Integer(6), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]), // index of constant 6
                    make(Opcode::Constant, vec![1]), // index of constant 2
                    make(Opcode::Div, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "1; 2".to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]), // index of constant 1
                    make(Opcode::Pop, vec![]),
                    make(Opcode::Constant, vec![1]), // index of constant 2
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "-1".to_string(),
                expected_constants: vec![Object::Integer(1)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]), // index of constant 1
                    make(Opcode::Minus, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_boolean_expressions() {
        let test_cases = vec![
            TestCase {
                input: "true".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![make(Opcode::True, vec![]), make(Opcode::Pop, vec![])],
            },
            TestCase {
                input: "false".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![make(Opcode::False, vec![]), make(Opcode::Pop, vec![])],
            },
            TestCase {
                input: "1 > 2".to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::GreaterThan, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "1 < 2".to_string(),
                expected_constants: vec![Object::Integer(2), Object::Integer(1)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::GreaterThan, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "1 == 2".to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Equal, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "1 != 2".to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::NotEqual, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "true == false".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::False, vec![]),
                    make(Opcode::Equal, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "true != false".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::False, vec![]),
                    make(Opcode::NotEqual, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "!true".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::Bang, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_conditionals() {
        let test_cases = vec![TestCase {
            input: "if (true) { 10 }; 3333;".to_string(),
            expected_constants: vec![Object::Integer(10), Object::Integer(3333)],
            expected_instructions: vec![
                make(Opcode::True, vec![]),
                make(Opcode::JumpNotTruthy, vec![7]), // jump to first pop
                make(Opcode::Constant, vec![0]),
                make(Opcode::Pop, vec![]), // 0007
                make(Opcode::Constant, vec![1]),
                make(Opcode::Pop, vec![]),
            ],
        }];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    fn run_compiler_test(test_case: TestCase) {
        let program = parse(test_case.input);
        let mut compiler = Compiler::new();
        let result = compiler.compile(program);

        match result {
            Err(err) => {
                assert!(false, "The compiler encountered an error: {}", err);
            }
            Ok(_) => {
                let bytecode = compiler.bytecode();
                test_instructions(test_case.expected_instructions, bytecode.instructions);
                test_constants(test_case.expected_constants, bytecode.constants)
            }
        }
    }

    fn parse(input: String) -> Program {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let result = parser.parse_program();

        match result {
            Ok(result) => result,
            Err(_) => {
                let errors = result.unwrap_err();
                let error_str = errors.join("\n");
                assert!(
                    false,
                    "The parser encountered {} errors:\n{}",
                    errors.len(),
                    error_str
                );
                panic!("unreachable");
            }
        }
    }

    fn test_instructions(expected: Vec<Instructions>, actual: Instructions) {
        let concat = expected.concat();

        if actual.len() != concat.len() {
            assert!(
                false,
                "Wrong instructions length\nexpected: {}\nactual: {}",
                stringify(concat).unwrap(),
                stringify(actual).unwrap()
            )
        }

        if !concat.iter().zip(actual.clone()).all(|(a, b)| *a == b) {
            assert!(
                false,
                "The actual instructions did not match the expected ones.\nexpected: {}\ngot: {}",
                stringify(concat).unwrap(),
                stringify(actual).unwrap()
            )
        }
    }

    fn test_constants(expected: Vec<Object>, actual: Vec<Object>) {
        if actual.len() != expected.len() {
            assert!(
                false,
                "Wrong amount of constants\nexpected: {}\nactual: {}",
                expected.len(),
                actual.len()
            )
        }

        if !expected.iter().zip(&actual).all(|(a, b)| *a == *b) {
            let expected_str = expected
                .iter()
                .map(|obj| obj.inspect())
                .collect::<Vec<String>>()
                .join(", ");
            let actual_str = actual
                .iter()
                .map(|obj| obj.inspect())
                .collect::<Vec<String>>()
                .join(", ");

            assert!(
                false,
                "The actual constants did not match the expected ones\nexpected: {}\nactual: {}",
                expected_str, actual_str
            )
        }
    }
}
