use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // identifiers, literals
    IDENT,
    INT,

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    EQ,
    NOTEQ,
    LT,
    GT,

    // delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tok_type: TokenType, literal: &str) -> Self {
        Self {
            tok_type,
            literal: String::from(literal),
        }
    }

    pub fn new_from_char(tok_type: TokenType, literal: char) -> Self {
        Self {
            tok_type,
            literal: literal.to_string(),
        }
    }

    pub fn lookup_ident_type(literal: &str) -> TokenType {
        return match literal {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        };
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::ILLEGAL => write!(f, "ILLEGAL"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::IDENT => write!(f, "IDENT"),
            TokenType::INT => write!(f, "INT"),
            TokenType::ASSIGN => write!(f, "="),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::BANG => write!(f, "!"),
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::EQ => write!(f, "=="),
            TokenType::NOTEQ => write!(f, "!="),
            TokenType::LT => write!(f, "<"),
            TokenType::GT => write!(f, ">"),
            TokenType::COMMA => write!(f, ","),
            TokenType::SEMICOLON => write!(f, ";"),
            TokenType::LPAREN => write!(f, "("),
            TokenType::RPAREN => write!(f, ")"),
            TokenType::LBRACE => write!(f, "{{"),
            TokenType::RBRACE => write!(f, "}}"),
            TokenType::FUNCTION => write!(f, "FUNCTION"),
            TokenType::LET => write!(f, "LET"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::IF => write!(f, "IF"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::RETURN => write!(f, "RETURN"),
        }
    }
}
