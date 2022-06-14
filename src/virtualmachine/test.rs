#[cfg(test)]
mod test {
    use crate::virtualmachine::code::{make, Instructions, OpcodeType};

    #[test]
    fn test_make() {
        struct TestCase {
            op: OpcodeType,
            operands: Vec<u32>,
            expected: Instructions,
        }

        let test_cases = vec![TestCase {
            op: OpcodeType::Constant,
            operands: vec![65534],
            expected: vec![0x01, 0xFF, 0xFE],
        }];

        for test_case in test_cases {
            let result = make(test_case.op, test_case.operands);
            assert_eq!(result.len(), test_case.expected.len());

            for (i, (actual, expected)) in result.iter().zip(test_case.expected.iter()).enumerate()
            {
                assert_eq!(actual, expected, "Byte {} differed!", i);
            }
        }
    }
}
