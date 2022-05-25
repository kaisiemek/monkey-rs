#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();

        let mut expected_strings: VecDeque<&str> =
            VecDeque::from_iter(["=", "+", "(", ")", "{", "}", ",", ";", ""]);

        let mut expected_types: VecDeque<TokenType> = VecDeque::from_iter([
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ]);

        let mut l = Lexer::new(input.to_string());
        while !expected_strings.is_empty() {
            let token = l.next_token();
            let expected_str = expected_strings.pop_front().unwrap().to_string();
            let expected_type = expected_types.pop_front().unwrap();
            assert_eq!(expected_type, token.tok_type);
            assert_eq!(expected_str, token.literal);
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = concat!(
            "let five = 5;\n",
            "let ten = 10;\n",
            "\n",
            "let add = fn(x, y) {\n",
            "  x + y;\n",
            "};\n",
            "\n",
            "let result = add(five, ten);\n"
        );

        let mut expected_strings: VecDeque<&str> = VecDeque::from_iter([
            "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fn",
            "(", "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=",
            "add", "(", "five", ",", "ten", ")", ";", "",
        ]);

        let mut expected_types: VecDeque<TokenType> = VecDeque::from_iter([
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::FUNCTION,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::COMMA,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::IDENT,
            TokenType::PLUS,
            TokenType::IDENT,
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::IDENT,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::COMMA,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ]);

        let mut l = Lexer::new(input.to_string());
        while !expected_strings.is_empty() {
            let token = l.next_token();
            let expected_str = expected_strings.pop_front().unwrap().to_string();
            let expected_type = expected_types.pop_front().unwrap();
            assert_eq!(expected_type, token.tok_type);
            assert_eq!(expected_str, token.literal);
        }
    }

    #[test]
    fn test_next_token_3() {
        let input = concat!(
            "let five = 5;\n",
            "let ten = 10;\n",
            "\n",
            "let add = fn(x, y) {\n",
            "  x + y;\n",
            "};\n",
            "\n",
            "let result = add(five, ten);\n",
            "!-/*5;\n",
            "5 < 10 > 5;\n",
            "\n",
            "if (5 < 10) {\n",
            "  return true;\n",
            "} else {\n",
            "  return false;\n",
            "}\n",
        );

        let mut expected_strings: VecDeque<&str> = VecDeque::from_iter([
            "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fn",
            "(", "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=",
            "add", "(", "five", ",", "ten", ")", ";", "!", "-", "/", "*", "5", ";", "5", "<", "10",
            ">", "5", ";", "if", "(", "5", "<", "10", ")", "{", "return", "true", ";", "}", "else",
            "{", "return", "false", ";", "}", "",
        ]);

        let mut expected_types: VecDeque<TokenType> = VecDeque::from_iter([
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::FUNCTION,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::COMMA,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::IDENT,
            TokenType::PLUS,
            TokenType::IDENT,
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::IDENT,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::COMMA,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::SEMICOLON,
            TokenType::BANG,
            TokenType::MINUS,
            TokenType::SLASH,
            TokenType::ASTERISK,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::INT,
            TokenType::LT,
            TokenType::INT,
            TokenType::GT,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::IF,
            TokenType::LPAREN,
            TokenType::INT,
            TokenType::LT,
            TokenType::INT,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RETURN,
            TokenType::TRUE,
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::ELSE,
            TokenType::LBRACE,
            TokenType::RETURN,
            TokenType::FALSE,
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::EOF,
        ]);

        let mut l = Lexer::new(input.to_string());
        while !expected_strings.is_empty() {
            let token = l.next_token();
            let expected_str = expected_strings.pop_front().unwrap().to_string();
            let expected_type = expected_types.pop_front().unwrap();
            assert_eq!(expected_type, token.tok_type);
            assert_eq!(expected_str, token.literal);
        }
    }

    #[test]
    fn test_string_splicing() {
        let input = "./--342TEST_IDENT//32".to_string();
        let mut cur_pos = 1;
        let mut cur_char = '\0';

        while !cur_char.is_ascii_alphabetic() {
            cur_pos += 1;
            cur_char = input.as_bytes()[cur_pos as usize] as char;
        }

        let start_pos = cur_pos;

        while cur_char.is_ascii_alphabetic() || cur_char == '_' {
            cur_pos += 1;
            cur_char = input.as_bytes()[cur_pos as usize] as char;
        }

        let res_str: String = input
            .chars()
            .skip(start_pos as usize)
            .take(cur_pos - start_pos as usize)
            .collect();
        assert_eq!(res_str, "TEST_IDENT");
    }
}
