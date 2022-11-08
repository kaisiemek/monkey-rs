#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        code::{make, stringify, Instructions, Opcode},
        compiler::{
            symbol_table::{Symbol, SymbolScope, SymbolTable},
            Compiler,
        },
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
        let test_cases = vec![
            TestCase {
                input: "if (true) { 10 }; 3333;".to_string(),
                expected_constants: vec![Object::Integer(10), Object::Integer(3333)],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::JumpNotTruthy, vec![10]),
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Jump, vec![11]),
                    make(Opcode::Null, vec![]), // 0010
                    make(Opcode::Pop, vec![]),  // 0011
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "if (true) { 10 } else { 20 }; 3333;".to_string(),
                expected_constants: vec![
                    Object::Integer(10),
                    Object::Integer(20),
                    Object::Integer(3333),
                ],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::JumpNotTruthy, vec![10]), // jump to 10
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Jump, vec![13]),    // jump to 13
                    make(Opcode::Constant, vec![1]), // 10
                    make(Opcode::Pop, vec![]),       // 13
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_global_let_statements() {
        let test_cases = vec![
            TestCase {
                input: concat!("let one = 1;\n", "let two = 2;").to_string(),
                expected_constants: vec![Object::Integer(1), Object::Integer(2)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::SetGlobal, vec![1]),
                ],
            },
            TestCase {
                input: concat!("let one = 1;\n", "one;").to_string(),
                expected_constants: vec![Object::Integer(1)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                    make(Opcode::GetGlobal, vec![0]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: concat!("let one = 1;\n", "let two = one;\n", "two;").to_string(),
                expected_constants: vec![Object::Integer(1)],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                    make(Opcode::GetGlobal, vec![0]),
                    make(Opcode::SetGlobal, vec![1]),
                    make(Opcode::GetGlobal, vec![1]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_symbol_table_define() {
        let expected: HashMap<String, Symbol> = HashMap::from([
            (
                "a".to_string(),
                Symbol {
                    name: "a".to_string(),
                    scope: SymbolScope::Global,
                    index: 0,
                },
            ),
            (
                "b".to_string(),
                Symbol {
                    name: "b".to_string(),
                    scope: SymbolScope::Global,
                    index: 1,
                },
            ),
        ]);

        let mut global = SymbolTable::new();

        let a = global.define("a");
        assert_eq!(a, &expected["a"]);

        let b = global.define("b");
        assert_eq!(b, &expected["b"]);
    }

    #[test]
    fn test_symbol_table_resolve_global() {
        let expected = [
            Symbol {
                name: "a".to_string(),
                scope: SymbolScope::Global,
                index: 0,
            },
            Symbol {
                name: "b".to_string(),
                scope: SymbolScope::Global,
                index: 1,
            },
        ];
        let mut global = SymbolTable::new();
        global.define("a");
        global.define("b");

        for sym in expected {
            let result = global.resolve(&sym.name);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), &sym);
        }
    }

    fn run_compiler_test(test_case: TestCase) {
        let program = parse(test_case.input);
        let mut compiler = Compiler::new();
        let result = compiler.compile_program(program);

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
