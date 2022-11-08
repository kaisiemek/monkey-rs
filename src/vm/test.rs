#[cfg(test)]
mod test {
    use crate::{
        compiler::Compiler,
        interpreter::object::Object,
        lexer::Lexer,
        parser::{ast::Program, Parser},
        vm::VM,
    };

    struct TestCase {
        input: String,
        expected: Object,
    }

    #[test]
    fn test_integer_arithmetic() {
        let test_cases = vec![
            TestCase {
                input: "1".to_string(),
                expected: Object::Integer(1),
            },
            TestCase {
                input: "2".to_string(),
                expected: Object::Integer(2),
            },
            TestCase {
                input: "1 + 2".to_string(),
                expected: Object::Integer(3),
            },
            TestCase {
                input: "1 - 2".to_string(),
                expected: Object::Integer(-1),
            },
            TestCase {
                input: "2 * 3".to_string(),
                expected: Object::Integer(6),
            },
            TestCase {
                input: "4 / 2".to_string(),
                expected: Object::Integer(2),
            },
            TestCase {
                input: "50 / 2 * 2 + 10 - 5".to_string(),
                expected: Object::Integer(55),
            },
            TestCase {
                input: "5 + 5 + 5 + 5 - 10".to_string(),
                expected: Object::Integer(10),
            },
            TestCase {
                input: "2 * 2 * 2 * 2 * 2".to_string(),
                expected: Object::Integer(32),
            },
            TestCase {
                input: "5 * 2 + 10".to_string(),
                expected: Object::Integer(20),
            },
            TestCase {
                input: "5 + 2 * 10".to_string(),
                expected: Object::Integer(25),
            },
            TestCase {
                input: "5 * (2 + 10)".to_string(),
                expected: Object::Integer(60),
            },
            TestCase {
                input: "-5".to_string(),
                expected: Object::Integer(-5),
            },
            TestCase {
                input: "-10".to_string(),
                expected: Object::Integer(-10),
            },
            TestCase {
                input: "-50 + 100 + -50".to_string(),
                expected: Object::Integer(0),
            },
            TestCase {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(),
                expected: Object::Integer(50),
            },
        ];

        for test_case in test_cases {
            run_vm_test(test_case);
        }
    }

    #[test]
    fn test_boolean_expressions() {
        let test_cases = vec![
            TestCase {
                input: "true".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "false".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "1 < 2".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "1 > 2".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "1 < 1".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "1 > 1".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "1 == 1".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "1 != 1".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "1 == 2".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "1 != 2".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "true == true".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "false == false".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "true == false".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "true != false".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "false != true".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "(1 < 2) == true".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "(1 < 2) == false".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "(1 > 2) == true".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "(1 > 2) == false".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "!true".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "!false".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "!5".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "!!true".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "!!false".to_string(),
                expected: Object::Boolean(false),
            },
            TestCase {
                input: "!!5".to_string(),
                expected: Object::Boolean(true),
            },
            TestCase {
                input: "!(if (false) { 5; })".to_string(),
                expected: Object::Boolean(true),
            },
        ];

        for test_case in test_cases {
            run_vm_test(test_case);
        }
    }

    #[test]
    fn test_conditionals() {
        let test_cases = vec![
            TestCase {
                input: "if (true) { 10 }".to_string(),
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (true) { 10 } else { 20 }".to_string(),
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (false) { 10 } else { 20 } ".to_string(),
                expected: Object::Integer(20),
            },
            TestCase {
                input: "if (1) { 10 }".to_string(),
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (1 < 2) { 10 }".to_string(),
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (1 < 2) { 10 } else { 20 }".to_string(),
                expected: Object::Integer(10),
            },
            TestCase {
                input: "if (1 > 2) { 10 } else { 20 }".to_string(),
                expected: Object::Integer(20),
            },
            TestCase {
                input: "if (1 > 2) { 10 }".to_string(),
                expected: Object::Null,
            },
            TestCase {
                input: "if (false) { 10 }".to_string(),
                expected: Object::Null,
            },
            TestCase {
                input: "if ((if (false) { 10 })) { 10 } else { 20 }".to_string(),
                expected: Object::Integer(20),
            },
        ];

        for test_case in test_cases {
            run_vm_test(test_case);
        }
    }

    #[test]
    fn test_global_let_statements() {
        let test_cases = vec![
            TestCase {
                input: "let one = 1; one".to_string(),
                expected: Object::Integer(1),
            },
            TestCase {
                input: "let one = 1; let two = 2; one + two".to_string(),
                expected: Object::Integer(3),
            },
            TestCase {
                input: "let one = 1; let two = one + one; one + two".to_string(),
                expected: Object::Integer(3),
            },
        ];

        for test_case in test_cases {
            run_vm_test(test_case);
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

    fn run_vm_test(test_case: TestCase) {
        let program = parse(test_case.input.clone());
        let mut compiler = Compiler::new();
        let comp_result = compiler.compile(program);

        if comp_result.is_err() {
            assert!(
                false,
                "An error occurred in the compiler: {}",
                comp_result.unwrap_err()
            );
        }

        let mut vm = VM::new();
        let vm_result = vm.run(compiler.bytecode());

        if vm_result.is_err() {
            assert!(
                false,
                "An error occurred in the VM: {}",
                vm_result.unwrap_err()
            );
        }

        let stack_element = vm.last_popped_stack_elem();
        assert_eq!(stack_element, test_case.expected);
    }
}
