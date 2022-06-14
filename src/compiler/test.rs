#[cfg(test)]
mod test {
    use crate::{
        compiler::Compiler,
        interpreter::object::Object,
        lexer::Lexer,
        parser::Parser,
        virtualmachine::code::{self, make, Instructions},
    };

    struct TestCase<'a> {
        input: &'a str,
        expected_constants: Vec<Object>,
        expected_instructions: Vec<Instructions>,
    }

    #[test]
    fn test_int_arithmetic() {
        let test_cases = vec![TestCase {
            input: "1 + 2",
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(code::OpcodeType::Constant, vec![0]),
                make(code::OpcodeType::Constant, vec![1]),
            ],
        }];

        for test_case in test_cases {
            run_compiler_tests(test_case);
        }
    }

    fn run_compiler_tests(test_case: TestCase) {
        let program = Parser::new(Lexer::new(test_case.input))
            .parse_program()
            .expect("parser errors occured");

        let compiler = Compiler::new();
        compiler
            .compile_program(program)
            .expect("compiler error occurred");

        let bytecode = compiler.bytecode();
        test_instructions(bytecode.instructions, test_case.expected_instructions);
        test_constants(bytecode.constants, test_case.expected_constants);
    }

    fn test_instructions(instructions: Instructions, expected: Vec<Instructions>) {
        let mut combined = Vec::new();

        for mut vec in expected {
            combined.append(&mut vec);
        }

        assert_eq!(instructions.len(), combined.len());

        instructions.iter().zip(combined.iter()).enumerate().all(
            |(index, (actual, expected))| -> bool {
                if actual == expected {
                    return true;
                } else {
                    panic!("code differs at {}: {} != {}", index, actual, expected);
                }
            },
        );
    }

    fn test_constants(constants: Vec<Object>, expected: Vec<Object>) {
        assert_eq!(constants.len(), expected.len());

        constants.iter().zip(expected.iter()).enumerate().all(
            |(index, (actual, expected))| -> bool {
                if actual == expected {
                    return true;
                } else {
                    panic!(
                        "constant differs at {}: {:?} != {:?}",
                        index, actual, expected
                    );
                }
            },
        );
    }
}
