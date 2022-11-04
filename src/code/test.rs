#[cfg(test)]
mod test {
    use crate::code::{make, read_operands, stringify, Instructions, Opcode};

    #[test]
    fn test_make_code() {
        struct Input {
            op: Opcode,
            operands: Vec<u16>,
        }

        struct TestCase {
            input: Input,
            expected: Vec<u8>,
        }

        let test_cases = vec![TestCase {
            input: Input {
                op: Opcode::Constant,
                operands: vec![0xFFFE], //65534
            },
            expected: vec![Opcode::Constant.into(), 0xFF, 0xFE],
        }];

        for test_case in test_cases {
            let result = make(test_case.input.op, test_case.input.operands);

            assert_eq!(result.len(), test_case.expected.len());
            for (res, exp) in result.iter().zip(test_case.expected.iter()) {
                assert_eq!(res, exp);
            }
        }
    }

    #[test]
    fn test_instructions_string() {
        let instructions = vec![
            make(Opcode::Constant, vec![1]),
            make(Opcode::Constant, vec![2]),
            make(Opcode::Constant, vec![65535]),
        ];

        let expected = concat!(
            "0000 Constant 1\n",
            "0003 Constant 2\n",
            "0006 Constant 65535\n",
        )
        .to_string();

        let concat: Instructions = instructions.concat();
        assert_eq!(stringify(concat).unwrap(), expected);
    }

    #[test]
    fn test_read_operands() {
        struct TestCase {
            op: Opcode,
            operands: Vec<u16>,
            bytes_read: u16,
        }

        let test_cases = vec![TestCase {
            op: Opcode::Constant,
            operands: vec![65535],
            bytes_read: 2,
        }];

        for test_case in test_cases {
            let instructions = make(test_case.op, test_case.operands.clone());

            let opcode: Result<Opcode, _> = instructions[0].try_into();
            assert!(opcode.is_ok());
            assert_eq!(opcode.clone().unwrap(), test_case.op);

            let (operands_read, n) = read_operands(opcode.unwrap(), &instructions[1..]);
            assert_eq!(n, test_case.bytes_read);
            assert_eq!(operands_read, test_case.operands);
        }
    }
}
