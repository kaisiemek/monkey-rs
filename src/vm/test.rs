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
                expected: Object::Integer(2),
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

        let mut vm = VM::new(compiler.bytecode());
        let vm_result = vm.run();

        if vm_result.is_err() {
            assert!(
                false,
                "An error occurred in the VM: {}",
                vm_result.unwrap_err()
            );
        }

        let stack_element = vm.stack_top();
        assert_eq!(stack_element, test_case.expected);
    }
}
