#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::lexer::{token::TokenType, Lexer};

    #[test]
    fn test_peek() {
        let input = "123";
        let mut l = Lexer::new(input);

        assert_eq!(l.peek(), '2');
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let mut expected_strings: VecDeque<&str> =
            VecDeque::from_iter(["=", "+", "(", ")", "{", "}", ",", ";", ""]);

        let mut expected_types: VecDeque<TokenType> = VecDeque::from_iter([
            TokenType::Assign,
            TokenType::Plus,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::Comma,
            TokenType::Semicolon,
            TokenType::EOF,
        ]);

        let mut l = Lexer::new(input);
        while !expected_strings.is_empty() {
            let token = l.next_token();
            let expected_str = expected_strings.pop_front().unwrap();
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
            "let result = add(five, ten);\n",
            "[1, \"test\", true]\n"
        );

        let mut expected_strings: VecDeque<&str> = VecDeque::from_iter([
            "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fn",
            "(", "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=",
            "add", "(", "five", ",", "ten", ")", ";", "[", "1", ",", "test", ",", "true", "]", "",
        ]);

        let mut expected_types: VecDeque<TokenType> = VecDeque::from_iter([
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Ident,
            TokenType::Plus,
            TokenType::Ident,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Ident,
            TokenType::LParen,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::LBracket,
            TokenType::Int,
            TokenType::Comma,
            TokenType::String,
            TokenType::Comma,
            TokenType::True,
            TokenType::RBracket,
            TokenType::EOF,
        ]);

        let mut l = Lexer::new(input);
        while !expected_strings.is_empty() {
            let token = l.next_token();
            let expected_str = expected_strings.pop_front().unwrap();
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
            "\n",
            "10 == 10;\n",
            "10 != 9;\n",
            "\"foobar\"\n",
            "\"foo bar\"\n",
            "\"\"\n",
            "{\"key\": \"value\", \"key2\": 42}\n",
        );

        let mut expected_strings: VecDeque<&str> = VecDeque::from_iter([
            "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fn",
            "(", "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=",
            "add", "(", "five", ",", "ten", ")", ";", "!", "-", "/", "*", "5", ";", "5", "<", "10",
            ">", "5", ";", "if", "(", "5", "<", "10", ")", "{", "return", "true", ";", "}", "else",
            "{", "return", "false", ";", "}", "10", "==", "10", ";", "10", "!=", "9", ";",
            "foobar", "foo bar", "", "{", "key", ":", "value", ",", "key2", ":", "42", "}", "",
        ]);

        let mut expected_types: VecDeque<TokenType> = VecDeque::from_iter([
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Ident,
            TokenType::Plus,
            TokenType::Ident,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Ident,
            TokenType::LParen,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::Int,
            TokenType::Lt,
            TokenType::Int,
            TokenType::Gt,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::If,
            TokenType::LParen,
            TokenType::Int,
            TokenType::Lt,
            TokenType::Int,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::True,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Else,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::False,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Int,
            TokenType::Eq,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::Int,
            TokenType::NotEq,
            TokenType::Int,
            TokenType::Semicolon,
            TokenType::String,
            TokenType::String,
            TokenType::String,
            TokenType::LBrace,
            TokenType::String,
            TokenType::Colon,
            TokenType::String,
            TokenType::Comma,
            TokenType::String,
            TokenType::Colon,
            TokenType::Int,
            TokenType::RBrace,
            TokenType::EOF,
        ]);

        let mut l = Lexer::new(input);
        while !expected_strings.is_empty() {
            let token = l.next_token();
            let expected_str = expected_strings.pop_front().unwrap();
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
