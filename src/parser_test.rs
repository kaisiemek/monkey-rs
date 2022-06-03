#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{ast::Program, lexer::Lexer, parser::Parser};

    #[test]
    fn test_next_token() {
        let input = concat!("let x = 5;\n", "let y = 10;\n", "let foobar = 838383;\n",);

        let mut expected_strings: VecDeque<&str> =
            VecDeque::from_iter(["=", "+", "(", ")", "{", "}", ",", ";", ""]);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program_result = parser.parse_program();
        assert!(
            matches!(program_result, Ok(Program { .. })),
            "Parser did not return a valid program"
        );
        let program = program_result.unwrap();

        assert_eq!(
            program.statements.len(),
            3,
            "Expected parser to parse 3 statements, parsed {} instead",
            program.statements.len()
        );

        // while !expected_strings.is_empty() {
        // let expected_str = expected_strings.pop_front().unwrap();
        // assert_eq!(expected_str, token./literal);
        // }
    }
}
