#[cfg(test)]
mod test {
    use crate::virtualmachine::code::{make, Opcode};

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
}
