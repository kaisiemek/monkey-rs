#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        rc::Rc,
        vec,
    };

    use crate::{
        code::{make, stringify, Instructions, Opcode},
        compiler::{
            symbol_table::{self, Symbol, SymbolScope, SymbolTable},
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
    fn test_string_expressions() {
        let test_cases = vec![
            TestCase {
                input: r#""monkey""#.to_string(),
                expected_constants: vec![Object::String("monkey".to_string())],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: r#""mon" + "key""#.to_string(),
                expected_constants: vec![
                    Object::String("mon".to_string()),
                    Object::String("key".to_string()),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Add, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_array_literals() {
        let test_cases = vec![
            TestCase {
                input: "[]".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::Array, vec![0]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "[1, 2, 3]".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Array, vec![3]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "[1 + 2, 3 - 4, 5 * 6]".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3),
                    Object::Integer(4),
                    Object::Integer(5),
                    Object::Integer(6),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Add, vec![]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Constant, vec![3]),
                    make(Opcode::Sub, vec![]),
                    make(Opcode::Constant, vec![4]),
                    make(Opcode::Constant, vec![5]),
                    make(Opcode::Mult, vec![]),
                    make(Opcode::Array, vec![3]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_hash_literals() {
        struct HashTestCase {
            input: String,
            expected_constants: HashSet<isize>,
            expected_instructions: HashSet<u8>,
        }

        let test_cases = vec![
            HashTestCase {
                input: "{}".to_string(),
                expected_constants: HashSet::new(),
                expected_instructions: vec![make(Opcode::Hash, vec![0]), make(Opcode::Pop, vec![])]
                    .concat()
                    .into_iter()
                    .collect(),
            },
            HashTestCase {
                input: "{1: 2, 3: 4, 5: 6}".to_string(),
                expected_constants: vec![1, 2, 3, 4, 5, 6].into_iter().collect(),
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Constant, vec![3]),
                    make(Opcode::Constant, vec![4]),
                    make(Opcode::Constant, vec![5]),
                    make(Opcode::Hash, vec![6]),
                    make(Opcode::Pop, vec![]),
                ]
                .concat()
                .into_iter()
                .collect(),
            },
            HashTestCase {
                input: "{1: 2 + 3, 4 * 5: 6}".to_string(),
                expected_constants: vec![1, 2, 3, 4, 5, 6].into_iter().collect(),
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Add, vec![]),
                    make(Opcode::Constant, vec![3]),
                    make(Opcode::Constant, vec![4]),
                    make(Opcode::Mult, vec![]),
                    make(Opcode::Constant, vec![5]),
                    make(Opcode::Hash, vec![4]),
                    make(Opcode::Pop, vec![]),
                ]
                .concat()
                .into_iter()
                .collect(),
            },
        ];

        // Little trickery as to not concern ourselves with HashMap order
        for test_case in test_cases {
            let program = parse(test_case.input);
            let mut compiler = Compiler::new();
            if let Err(err) = compiler.compile_program(program) {
                panic!("The compiler encountered an error {}", err);
            }
            let bytecode = compiler.bytecode();
            assert_eq!(
                test_case.expected_instructions,
                bytecode.instructions.into_iter().collect()
            );

            let mut constants = vec![];
            for c in bytecode.constants {
                if let Object::Integer(int) = c {
                    constants.push(int);
                } else {
                    panic!("Unexpected Object");
                }
            }

            assert_eq!(
                test_case.expected_constants,
                constants.into_iter().collect()
            );
        }
    }

    #[test]
    fn test_index_expressions() {
        let test_cases = vec![
            TestCase {
                input: "[1, 2, 3][1 + 1]".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3),
                    Object::Integer(1),
                    Object::Integer(1),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Array, vec![3]),
                    make(Opcode::Constant, vec![3]),
                    make(Opcode::Constant, vec![4]),
                    make(Opcode::Add, vec![]),
                    make(Opcode::Index, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "{1: 2}[2 - 1]".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(2),
                    Object::Integer(1),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Hash, vec![2]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Constant, vec![3]),
                    make(Opcode::Sub, vec![]),
                    make(Opcode::Index, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_functions() {
        let test_cases = vec![
            TestCase {
                input: "fn() { return 5 + 10; }".to_string(),
                expected_constants: vec![
                    Object::Integer(5),
                    Object::Integer(10),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::Constant, vec![1]),
                            make(Opcode::Add, vec![]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "fn() { 5 + 10 }".to_string(),
                expected_constants: vec![
                    Object::Integer(5),
                    Object::Integer(10),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::Constant, vec![1]),
                            make(Opcode::Add, vec![]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "fn() { 1; 2 }".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::Pop, vec![]),
                            make(Opcode::Constant, vec![1]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "fn() { }".to_string(),
                expected_constants: vec![Object::CompiledFunction(
                    vec![make(Opcode::Return, vec![])].concat(),
                )],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_function_calls() {
        let test_cases = vec![
            TestCase {
                input: "fn() { 24 }();".to_string(),
                expected_constants: vec![
                    Object::Integer(24),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Call, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "let noArg = fn() { 24 }; noArg();".to_string(),
                expected_constants: vec![
                    Object::Integer(24),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::SetGlobal, vec![0]),
                    make(Opcode::GetGlobal, vec![0]),
                    make(Opcode::Call, vec![]),
                    make(Opcode::Pop, vec![]),
                ],
            },
        ];

        for test_case in test_cases {
            run_compiler_test(test_case);
        }
    }

    #[test]
    fn test_let_statements_scopes() {
        let test_cases = vec![
            TestCase {
                input: "let num = 55; fn() { num }".to_string(),
                expected_constants: vec![
                    Object::Integer(55),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::GetGlobal, vec![0]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "fn() { let num = 55; num; }".to_string(),
                expected_constants: vec![
                    Object::Integer(55),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::SetLocal, vec![0]),
                            make(Opcode::GetLocal, vec![0]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Pop, vec![]),
                ],
            },
            TestCase {
                input: "fn() { let a = 55; let b = 77; a + b; }".to_string(),
                expected_constants: vec![
                    Object::Integer(55),
                    Object::Integer(77),
                    Object::CompiledFunction(
                        vec![
                            make(Opcode::Constant, vec![0]),
                            make(Opcode::SetLocal, vec![0]),
                            make(Opcode::Constant, vec![1]),
                            make(Opcode::SetLocal, vec![1]),
                            make(Opcode::GetLocal, vec![0]),
                            make(Opcode::GetLocal, vec![1]),
                            make(Opcode::Add, vec![]),
                            make(Opcode::ReturnValue, vec![]),
                        ]
                        .concat(),
                    ),
                ],
                expected_instructions: vec![
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
    fn test_compiler_scopes() {
        let mut compiler = Compiler::new();
        assert_eq!(compiler.scope_index, 0);

        let symbol_table = compiler.symbol_table.clone();
        compiler.emit(Opcode::Mult, vec![]);

        compiler.enter_scope();
        compiler.emit(Opcode::Sub, vec![]);
        assert_eq!(compiler.scopes[compiler.scope_index].instructions.len(), 1);
        assert_eq!(
            compiler.scopes[compiler.scope_index]
                .last_instruction
                .opcode,
            Opcode::Sub
        );

        assert_eq!(
            compiler.symbol_table.outer.as_ref().unwrap().as_ref(),
            &symbol_table
        );

        compiler.leave_scope();
        assert_eq!(compiler.scope_index, 0);
        assert_eq!(&compiler.symbol_table, &symbol_table);
        assert!(compiler.symbol_table.outer.as_ref().is_none());

        compiler.emit(Opcode::Add, vec![]);
        assert_eq!(compiler.scopes[compiler.scope_index].instructions.len(), 2);
        assert_eq!(
            compiler.scopes[compiler.scope_index]
                .last_instruction
                .opcode,
            Opcode::Add
        );
        assert_eq!(
            compiler.scopes[compiler.scope_index]
                .previous_instruction
                .opcode,
            Opcode::Mult
        );
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
            (
                "c".to_string(),
                Symbol {
                    name: "c".to_string(),
                    scope: SymbolScope::Local,
                    index: 0,
                },
            ),
            (
                "d".to_string(),
                Symbol {
                    name: "d".to_string(),
                    scope: SymbolScope::Local,
                    index: 1,
                },
            ),
            (
                "e".to_string(),
                Symbol {
                    name: "e".to_string(),
                    scope: SymbolScope::Local,
                    index: 0,
                },
            ),
            (
                "f".to_string(),
                Symbol {
                    name: "f".to_string(),
                    scope: SymbolScope::Local,
                    index: 1,
                },
            ),
        ]);

        let mut global = SymbolTable::new();

        let a = global.define("a");
        assert_eq!(a, &expected["a"]);

        let b = global.define("b");
        assert_eq!(b, &expected["b"]);

        let mut first_local = SymbolTable::with_enclosed(Rc::from(global));
        let c = first_local.define("c");
        assert_eq!(c, &expected["c"]);

        let d = first_local.define("d");
        assert_eq!(d, &expected["d"]);

        let mut second_local = SymbolTable::with_enclosed(Rc::from(first_local));
        let e = second_local.define("e");
        assert_eq!(e, &expected["e"]);

        let f = second_local.define("f");
        assert_eq!(f, &expected["f"]);
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

    #[test]
    fn test_symbol_table_resolve_local() {
        let mut global = SymbolTable::new();
        global.define("a");
        global.define("b");

        let mut local = SymbolTable::with_enclosed(Rc::from(global));
        local.define("c");
        local.define("d");

        let expected = vec![
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
            Symbol {
                name: "c".to_string(),
                scope: SymbolScope::Local,
                index: 0,
            },
            Symbol {
                name: "d".to_string(),
                scope: SymbolScope::Local,
                index: 1,
            },
        ];

        for symbol in expected {
            let Some(resolved_symbol) = local.resolve(&symbol.name) else {
                panic!("Can not resolve {}", symbol.name);
            };

            assert_eq!(symbol, *resolved_symbol);
        }
    }

    #[test]
    fn test_symbol_table_resolve_nested_local() {
        struct TestCase {
            table: Rc<SymbolTable>,
            expected_symbols: Vec<Symbol>,
        }

        let mut global = SymbolTable::new();
        global.define("a");
        global.define("b");

        let mut first_local = SymbolTable::with_enclosed(Rc::new(global));
        first_local.define("c");
        first_local.define("d");

        let first_local_rc = Rc::new(first_local);

        let mut second_local = SymbolTable::with_enclosed(first_local_rc.clone());
        second_local.define("e");
        second_local.define("f");

        let test_cases = vec![
            TestCase {
                table: first_local_rc,
                expected_symbols: vec![
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
                    Symbol {
                        name: "c".to_string(),
                        scope: SymbolScope::Local,
                        index: 0,
                    },
                    Symbol {
                        name: "d".to_string(),
                        scope: SymbolScope::Local,
                        index: 1,
                    },
                ],
            },
            TestCase {
                table: Rc::from(second_local),
                expected_symbols: vec![
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
                    Symbol {
                        name: "c".to_string(),
                        scope: SymbolScope::Local,
                        index: 0,
                    },
                    Symbol {
                        name: "d".to_string(),
                        scope: SymbolScope::Local,
                        index: 1,
                    },
                    Symbol {
                        name: "e".to_string(),
                        scope: SymbolScope::Local,
                        index: 0,
                    },
                    Symbol {
                        name: "f".to_string(),
                        scope: SymbolScope::Local,
                        index: 1,
                    },
                ],
            },
        ];

        for test_case in test_cases {
            let table = test_case.table;
            let expected = test_case.expected_symbols;

            for symbol in expected {
                let Some(resolved_symbol) = table.resolve(&symbol.name) else {
                panic!("Can not resolve {}", symbol.name);
            };
                assert_eq!(symbol, *resolved_symbol);
            }
        }
    }

    fn run_compiler_test(test_case: TestCase) {
        let program = parse(test_case.input);
        let mut compiler = Compiler::new();
        if let Err(err) = compiler.compile_program(program) {
            panic!("The compiler encountered an error {}", err);
        }
        let bytecode = compiler.bytecode();
        test_instructions(test_case.expected_instructions, bytecode.instructions);
        test_constants(test_case.expected_constants, bytecode.constants)
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
