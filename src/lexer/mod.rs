use self::token::{Token, TokenType};

mod test;
pub mod token;

pub struct Lexer {
    input: String,
    position: i64,
    read_pos: i64,
    cur_char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut new_lexer: Self = Self {
            input: String::from(input),
            position: 0,
            read_pos: 0,
            cur_char: '\0',
        };

        new_lexer.read_char();
        return new_lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut skip_next_read = false;

        let tok: Token = match self.cur_char {
            '+' => Token::new_from_char(TokenType::PLUS, self.cur_char),
            '-' => Token::new_from_char(TokenType::MINUS, self.cur_char),
            '/' => Token::new_from_char(TokenType::SLASH, self.cur_char),
            '*' => Token::new_from_char(TokenType::ASTERISK, self.cur_char),
            '<' => Token::new_from_char(TokenType::LT, self.cur_char),
            '>' => Token::new_from_char(TokenType::GT, self.cur_char),
            ';' => Token::new_from_char(TokenType::SEMICOLON, self.cur_char),
            ',' => Token::new_from_char(TokenType::COMMA, self.cur_char),
            '(' => Token::new_from_char(TokenType::LPAREN, self.cur_char),
            ')' => Token::new_from_char(TokenType::RPAREN, self.cur_char),
            '{' => Token::new_from_char(TokenType::LBRACE, self.cur_char),
            '}' => Token::new_from_char(TokenType::RBRACE, self.cur_char),
            '[' => Token::new_from_char(TokenType::LBRACKET, self.cur_char),
            ']' => Token::new_from_char(TokenType::RBRACKET, self.cur_char),
            '\0' => Token::new(TokenType::EOF, ""),
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::EQ, "==")
                } else {
                    Token::new_from_char(TokenType::ASSIGN, self.cur_char)
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::NOTEQ, "!=")
                } else {
                    Token::new_from_char(TokenType::BANG, self.cur_char)
                }
            }
            '"' => {
                let literal = self.read_string();
                Token::new(TokenType::STRING, &literal)
            }
            _ => {
                if self.cur_char.is_ascii_alphabetic() || self.cur_char == '_' {
                    let literal = self.read_identifier();
                    let tok_type = Token::lookup_ident_type(&literal);
                    skip_next_read = true;
                    Token::new(tok_type, &literal)
                } else if self.cur_char.is_ascii_digit() {
                    skip_next_read = true;
                    Token::new(TokenType::INT, &self.read_number())
                } else {
                    Token::new_from_char(TokenType::ILLEGAL, self.cur_char)
                }
            }
        };
        if !skip_next_read {
            self.read_char();
        }
        return tok;
    }
}

// Internal methods
impl Lexer {
    fn read_char(&mut self) {
        if self.read_pos as usize >= self.input.len() {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.input.as_bytes()[self.read_pos as usize] as char;
        }

        self.position = self.read_pos;
        self.read_pos += 1;
    }

    fn peek(&mut self) -> char {
        if self.read_pos as usize >= self.input.len() {
            return '\0';
        }

        return self.input.as_bytes()[(self.read_pos) as usize] as char;
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.cur_char.is_ascii_alphabetic() || self.cur_char == '_' {
            self.read_char();
        }
        // String splicing, very convoluted?
        // Get all the chars as an iterable object, skip the first pos chars,
        // then take the next current position - pos chars and collect them
        // into a string.
        return self
            .input
            .chars()
            .skip(pos as usize)
            .take((self.position - pos) as usize)
            .collect();
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.cur_char.is_ascii_digit() {
            self.read_char();
        }
        // String splicing, very convoluted?
        // Get all the chars as an iterable object, skip the first pos chars,
        // then take the next current position - pos chars and collect them
        // into a string.
        return self
            .input
            .chars()
            .skip(pos as usize)
            .take((self.position - pos) as usize)
            .collect();
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let pos = self.position;

        while self.cur_char != '"' && self.cur_char != '\0' {
            self.read_char();
        }

        return self
            .input
            .chars()
            .skip(pos as usize)
            .take((self.position - pos) as usize)
            .collect();
    }
}
