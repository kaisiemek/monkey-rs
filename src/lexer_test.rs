#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{lexer::Lexer, token::Token, token::TokenType};

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let mut expected: VecDeque<Token> = VecDeque::from_iter([
            Token {
                literal: "=".to_string(),
                tok_type: TokenType::ASSIGN,
            },
            Token {
                literal: "+".to_string(),
                tok_type: TokenType::PLUS,
            },
            Token {
                literal: "(".to_string(),
                tok_type: TokenType::LPAREN,
            },
            Token {
                literal: ")".to_string(),
                tok_type: TokenType::RPAREN,
            },
            Token {
                literal: "{".to_string(),
                tok_type: TokenType::LBRACE,
            },
            Token {
                literal: "}".to_string(),
                tok_type: TokenType::RBRACE,
            },
            Token {
                literal: ",".to_string(),
                tok_type: TokenType::COMMA,
            },
            Token {
                literal: ";".to_string(),
                tok_type: TokenType::SEMICOLON,
            },
        ]);

        let mut l = Lexer::new(input);
        while !expected.is_empty() {
            let token = l.next_token();
            let cur_expected = expected.pop_front().unwrap();
            assert_eq!(cur_expected.literal, token.literal);
            assert_eq!(cur_expected.tok_type, token.tok_type);
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

        let mut expected: VecDeque<Token> = VecDeque::from_iter([
            Token {
                literal: "let".to_string(),
                tok_type: TokenType::LET,
            },
            Token {
                literal: "five".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: "=".to_string(),
                tok_type: TokenType::ASSIGN,
            },
            Token {
                literal: "5".to_string(),
                tok_type: TokenType::INT,
            },
            Token {
                literal: ";".to_string(),
                tok_type: TokenType::SEMICOLON,
            },
            Token {
                literal: "let".to_string(),
                tok_type: TokenType::LET,
            },
            Token {
                literal: "ten".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: "=".to_string(),
                tok_type: TokenType::ASSIGN,
            },
            Token {
                literal: "10".to_string(),
                tok_type: TokenType::INT,
            },
            Token {
                literal: ";".to_string(),
                tok_type: TokenType::SEMICOLON,
            },
            Token {
                literal: "let".to_string(),
                tok_type: TokenType::LET,
            },
            Token {
                literal: "add".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: "=".to_string(),
                tok_type: TokenType::ASSIGN,
            },
            Token {
                literal: "fn".to_string(),
                tok_type: TokenType::FUNCTION,
            },
            Token {
                literal: "(".to_string(),
                tok_type: TokenType::LPAREN,
            },
            Token {
                literal: "x".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: ",".to_string(),
                tok_type: TokenType::COMMA,
            },
            Token {
                literal: "y".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: ")".to_string(),
                tok_type: TokenType::RPAREN,
            },
            Token {
                literal: "{".to_string(),
                tok_type: TokenType::LBRACE,
            },
            Token {
                literal: "x".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: "+".to_string(),
                tok_type: TokenType::PLUS,
            },
            Token {
                literal: "y".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: ";".to_string(),
                tok_type: TokenType::SEMICOLON,
            },
            Token {
                literal: "}".to_string(),
                tok_type: TokenType::RBRACE,
            },
            Token {
                literal: ";".to_string(),
                tok_type: TokenType::SEMICOLON,
            },
            Token {
                literal: "let".to_string(),
                tok_type: TokenType::LET,
            },
            Token {
                literal: "result".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: "=".to_string(),
                tok_type: TokenType::ASSIGN,
            },
            Token {
                literal: "add".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: "(".to_string(),
                tok_type: TokenType::LPAREN,
            },
            Token {
                literal: "five".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: ",".to_string(),
                tok_type: TokenType::COMMA,
            },
            Token {
                literal: "ten".to_string(),
                tok_type: TokenType::IDENT,
            },
            Token {
                literal: ")".to_string(),
                tok_type: TokenType::RPAREN,
            },
            Token {
                literal: ";".to_string(),
                tok_type: TokenType::SEMICOLON,
            },
            Token {
                literal: "".to_string(),
                tok_type: TokenType::EOF,
            },
        ]);

        let mut l = Lexer::new(input.to_string());
        while !expected.is_empty() {
            let token = l.next_token();
            let cur_expected = expected.pop_front().unwrap();
            assert_eq!(cur_expected.tok_type, token.tok_type);
            assert_eq!(cur_expected.literal, token.literal);
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
